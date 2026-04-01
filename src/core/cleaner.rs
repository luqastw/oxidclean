//! Limpador de pacotes

use crate::dal::{ConfigReader, OperationLogger};
use crate::models::{Config, Package};
use crate::utils::{permissions, progress, symbols};
use crate::{OxidCleanError, Result};
use colored::Colorize;
use dialoguer::Confirm;
use log::{debug, info, warn};
use std::process::Command;

/// Resultado da operação de limpeza
#[derive(Debug, Clone)]
pub struct CleanResult {
    /// Pacotes removidos com sucesso
    pub removed: Vec<String>,

    /// Pacotes que foram pulados
    pub skipped: Vec<String>,

    /// Pacotes que falharam na remoção
    pub failed: Vec<(String, String)>, // (nome, erro)

    /// Espaço total liberado em bytes
    pub space_freed: u64,
}

impl CleanResult {
    fn new() -> Self {
        Self {
            removed: Vec::new(),
            skipped: Vec::new(),
            failed: Vec::new(),
            space_freed: 0,
        }
    }
}

/// Limpador de pacotes órfãos
pub struct Cleaner {
    config: Config,
    logger: OperationLogger,
    dry_run: bool,
    auto_confirm: bool,
}

impl Cleaner {
    /// Cria um novo cleaner
    pub fn new(dry_run: bool, auto_confirm: bool) -> Result<Self> {
        Ok(Self {
            config: ConfigReader::load(),
            logger: OperationLogger::new()?,
            dry_run,
            auto_confirm,
        })
    }

    /// Remove pacotes de forma interativa
    pub fn clean_interactive(&mut self, packages: Vec<Package>) -> Result<CleanResult> {
        let mut result = CleanResult::new();

        if packages.is_empty() {
            info!("Nenhum pacote para remover");
            return Ok(result);
        }

        // Verificar permissões se não for dry-run
        if !self.dry_run {
            permissions::ensure_root()?;
        }

        let total = packages.len();
        info!("Processando {} pacotes...", total);

        // Criar progress bar se não for auto_confirm (modo interativo tem seu próprio feedback)
        let pb = if self.auto_confirm && !self.dry_run {
            Some(progress::create_clean_progress(total as u64))
        } else {
            None
        };

        for pkg in packages.into_iter() {
            if let Some(ref pb) = pb {
                pb.set_message(pkg.name.to_string());
            }

            // Verificar se é protegido
            if self.config.is_protected(&pkg.name) {
                let msg = format!("{} {} (protegido)", symbols::WARNING.yellow(), pkg.name);
                if let Some(ref pb) = pb {
                    pb.println(msg);
                } else {
                    println!("{}", msg);
                }
                warn!("Pacote '{}' está protegido, pulando", pkg.name);
                result.skipped.push(pkg.name.clone());
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
                continue;
            }

            // Verificar se está na lista de ignorados
            if self.config.is_ignored(&pkg.name) {
                let msg = format!("{} {} (ignorado)", symbols::WARNING.yellow(), pkg.name);
                if let Some(ref pb) = pb {
                    pb.println(msg);
                } else if !self.auto_confirm {
                    println!("{}", msg);
                }
                debug!("Pacote '{}' está na lista de ignorados, pulando", pkg.name);
                result.skipped.push(pkg.name.clone());
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
                continue;
            }

            // Solicitar confirmação
            if !self.auto_confirm && !self.confirm_removal(&pkg)? {
                println!(
                    "{} {} (usuário recusou)",
                    symbols::WARNING.yellow(),
                    pkg.name
                );
                result.skipped.push(pkg.name.clone());
                continue;
            }

            // Executar remoção
            match self.remove_package(&pkg) {
                Ok(()) => {
                    let msg = format!(
                        "{} {} removido ({})",
                        symbols::SUCCESS.green(),
                        pkg.name,
                        crate::utils::humanize_bytes(pkg.size)
                    );
                    if let Some(ref pb) = pb {
                        pb.println(msg);
                    } else if !self.dry_run {
                        println!("{}", msg);
                    }
                    result.removed.push(pkg.name.clone());
                    result.space_freed += pkg.size;
                }
                Err(e) => {
                    let msg = format!("{} {} falhou: {}", symbols::ERROR.red(), pkg.name, e);
                    if let Some(ref pb) = pb {
                        pb.println(msg);
                    } else {
                        println!("{}", msg);
                    }
                    result.failed.push((pkg.name.clone(), e.to_string()));
                }
            }

            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        Ok(result)
    }

    /// Solicita confirmação para remoção
    fn confirm_removal(&self, pkg: &Package) -> Result<bool> {
        if self.dry_run {
            println!(
                "[DRY-RUN] Removeria: {} ({}, {})",
                pkg.name,
                pkg.version,
                crate::utils::humanize_bytes(pkg.size)
            );
            return Ok(true);
        }

        let prompt = format!(
            "Remover '{}' ({}, {})?",
            pkg.name,
            pkg.version,
            crate::utils::humanize_bytes(pkg.size)
        );

        Confirm::new()
            .with_prompt(prompt)
            .default(false)
            .interact()
            .map_err(|_| OxidCleanError::OperationCancelled)
    }

    /// Remove um pacote usando pacman
    fn remove_package(&mut self, pkg: &Package) -> Result<()> {
        if self.dry_run {
            info!("[DRY-RUN] Removeria pacote '{}'", pkg.name);
            return Ok(());
        }

        info!("Removendo pacote '{}'...", pkg.name);

        let output = Command::new("pacman")
            .args(["-Rns", "--noconfirm", &pkg.name])
            .output()?;

        let success = output.status.success();

        // Logar operação
        self.logger.log_removal(pkg, success)?;

        if success {
            info!("Pacote '{}' removido com sucesso", pkg.name);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(OxidCleanError::PacmanExecError(format!(
                "Falha ao remover '{}': {}",
                pkg.name, stderr
            )))
        }
    }

    /// Verifica se um pacote está protegido
    pub fn is_protected(&self, package_name: &str) -> bool {
        self.config.is_protected(package_name)
    }

    /// Retorna se está em modo dry-run
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_result_new() {
        let result = CleanResult::new();
        assert!(result.removed.is_empty());
        assert!(result.skipped.is_empty());
        assert!(result.failed.is_empty());
        assert_eq!(result.space_freed, 0);
    }

    #[test]
    fn test_is_protected() {
        let cleaner = Cleaner::new(true, false).unwrap();
        assert!(cleaner.is_protected("base"));
        assert!(cleaner.is_protected("pacman"));
        assert!(!cleaner.is_protected("vim"));
    }
}
