//! Leitor do banco de dados do Pacman

use crate::error::{OxidCleanError, Result};
use crate::models::{InstallReason, Package};
use chrono::{TimeZone, Utc};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Caminho padrão do banco de dados do Pacman
pub const PACMAN_DB_PATH: &str = "/var/lib/pacman/local";

/// Leitor do banco de dados do Pacman
pub struct PacmanReader {
    db_path: PathBuf,
}

impl PacmanReader {
    /// Cria um novo leitor do banco de dados do Pacman
    pub fn new() -> Result<Self> {
        Self::with_path(PACMAN_DB_PATH)
    }

    /// Cria um leitor com caminho customizado (útil para testes)
    pub fn with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db_path = path.as_ref().to_path_buf();

        if !db_path.exists() {
            return Err(OxidCleanError::PacmanDbNotFound(db_path));
        }

        if !db_path.is_dir() {
            return Err(OxidCleanError::PacmanDbReadError(
                "Caminho do banco de dados não é um diretório".to_string(),
            ));
        }

        Ok(Self { db_path })
    }

    /// Valida a integridade básica do banco de dados
    pub fn validate_integrity(&self) -> Result<()> {
        // Verificar se o diretório existe e é acessível
        if !self.db_path.exists() {
            return Err(OxidCleanError::PacmanDbNotFound(self.db_path.clone()));
        }

        // Verificar se podemos ler o diretório
        fs::read_dir(&self.db_path).map_err(|e| {
            OxidCleanError::PacmanDbReadError(format!(
                "Não foi possível ler o diretório do banco de dados: {}",
                e
            ))
        })?;

        // Verificar se existe pelo menos um pacote
        let entries: Vec<_> = fs::read_dir(&self.db_path)
            .map_err(|e| OxidCleanError::PacmanDbReadError(e.to_string()))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();

        if entries.is_empty() {
            return Err(OxidCleanError::PacmanDbReadError(
                "Banco de dados vazio - nenhum pacote encontrado".to_string(),
            ));
        }

        Ok(())
    }

    /// Lê todos os pacotes instalados do banco de dados
    pub fn read_all_packages(&self) -> Result<Vec<Package>> {
        let entries: Vec<PathBuf> = fs::read_dir(&self.db_path)
            .map_err(|e| OxidCleanError::PacmanDbReadError(e.to_string()))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();

        // Usar rayon para parsing paralelo
        let packages: Vec<Package> = entries
            .par_iter()
            .filter_map(|path| match self.read_package_from_dir(path) {
                Ok(pkg) => Some(pkg),
                Err(e) => {
                    log::warn!("Falha ao ler pacote de {}: {}", path.display(), e);
                    None
                }
            })
            .collect();

        Ok(packages)
    }

    /// Lê informações de um pacote específico pelo nome
    pub fn read_package(&self, name: &str) -> Result<Package> {
        // Encontrar o diretório do pacote (nome-versão)
        let entries = fs::read_dir(&self.db_path)
            .map_err(|e| OxidCleanError::PacmanDbReadError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                // O diretório é nomeado como "pacote-versão"
                // Precisamos verificar se começa com o nome do pacote
                if dir_name.starts_with(&format!("{}-", name)) || dir_name == name {
                    if let Ok(pkg) = self.read_package_from_dir(&path) {
                        if pkg.name == name {
                            return Ok(pkg);
                        }
                    }
                }
            }
        }

        Err(OxidCleanError::PackageNotFound(name.to_string()))
    }

    /// Lê um pacote de um diretório específico
    fn read_package_from_dir(&self, dir: &Path) -> Result<Package> {
        let desc_path = dir.join("desc");

        if !desc_path.exists() {
            let dir_name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            return Err(OxidCleanError::PackageParseError(
                dir_name.to_string(),
                "Arquivo 'desc' não encontrado".to_string(),
            ));
        }

        let desc_content = fs::read_to_string(&desc_path).map_err(|e| {
            OxidCleanError::PackageParseError(
                dir.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string(),
                e.to_string(),
            )
        })?;

        self.parse_desc(&desc_content, dir)
    }

    /// Parse do arquivo desc
    fn parse_desc(&self, content: &str, dir: &Path) -> Result<Package> {
        let mut name = String::new();
        let mut version = String::new();
        let mut size: u64 = 0;
        let mut install_reason = InstallReason::Explicit;
        let mut dependencies = Vec::new();
        let mut optional_deps = Vec::new();
        let mut install_date = None;
        let mut description = None;
        let mut arch = None;
        let mut url = None;

        let mut current_section = String::new();
        let mut section_lines: Vec<String> = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('%') && line.ends_with('%') {
                // Processar seção anterior
                self.process_section(
                    &current_section,
                    &section_lines,
                    &mut name,
                    &mut version,
                    &mut size,
                    &mut install_reason,
                    &mut dependencies,
                    &mut optional_deps,
                    &mut install_date,
                    &mut description,
                    &mut arch,
                    &mut url,
                );

                // Nova seção
                current_section = line[1..line.len() - 1].to_string();
                section_lines.clear();
            } else if !line.is_empty() {
                section_lines.push(line.to_string());
            }
        }

        // Processar última seção
        self.process_section(
            &current_section,
            &section_lines,
            &mut name,
            &mut version,
            &mut size,
            &mut install_reason,
            &mut dependencies,
            &mut optional_deps,
            &mut install_date,
            &mut description,
            &mut arch,
            &mut url,
        );

        if name.is_empty() || version.is_empty() {
            let dir_name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            return Err(OxidCleanError::PackageParseError(
                dir_name.to_string(),
                "Nome ou versão não encontrados".to_string(),
            ));
        }

        Ok(Package {
            name,
            version,
            size,
            install_reason,
            dependencies,
            optional_deps,
            install_date,
            description,
            arch,
            url,
        })
    }

    /// Processa uma seção do arquivo desc
    #[allow(clippy::too_many_arguments)]
    fn process_section(
        &self,
        section: &str,
        lines: &[String],
        name: &mut String,
        version: &mut String,
        size: &mut u64,
        install_reason: &mut InstallReason,
        dependencies: &mut Vec<String>,
        optional_deps: &mut Vec<String>,
        install_date: &mut Option<chrono::DateTime<Utc>>,
        description: &mut Option<String>,
        arch: &mut Option<String>,
        url: &mut Option<String>,
    ) {
        if lines.is_empty() {
            return;
        }

        match section {
            "NAME" => *name = lines[0].clone(),
            "VERSION" => *version = lines[0].clone(),
            "SIZE" => {
                *size = lines[0].parse().unwrap_or(0);
            }
            "ISIZE" => {
                // Installed size em bytes
                *size = lines[0].parse().unwrap_or(0);
            }
            "REASON" => {
                *install_reason = match lines[0].as_str() {
                    "1" => InstallReason::Dependency,
                    _ => InstallReason::Explicit,
                };
            }
            "DEPENDS" => {
                *dependencies = lines
                    .iter()
                    .map(|s| {
                        // Remover versão da dependência (ex: "glibc>=2.38" -> "glibc")
                        s.split(['>', '<', '=']).next().unwrap_or(s).to_string()
                    })
                    .collect();
            }
            "OPTDEPENDS" => {
                *optional_deps = lines
                    .iter()
                    .map(|s| {
                        // Formato: "pacote: descrição"
                        s.split(':').next().unwrap_or(s).trim().to_string()
                    })
                    .collect();
            }
            "INSTALLDATE" => {
                if let Ok(timestamp) = lines[0].parse::<i64>() {
                    *install_date = Utc.timestamp_opt(timestamp, 0).single();
                }
            }
            "DESC" => {
                *description = Some(lines.join(" "));
            }
            "ARCH" => {
                *arch = Some(lines[0].clone());
            }
            "URL" => {
                *url = Some(lines[0].clone());
            }
            _ => {}
        }
    }

    /// Retorna o caminho do banco de dados
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }
}

impl Default for PacmanReader {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(PACMAN_DB_PATH),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_mock_package(dir: &Path, name: &str, version: &str, reason: &str) {
        let pkg_dir = dir.join(format!("{}-{}", name, version));
        fs::create_dir_all(&pkg_dir).unwrap();

        let desc_content = format!(
            r#"%NAME%
{}

%VERSION%
{}

%DESC%
Test package

%ISIZE%
1024

%REASON%
{}

%DEPENDS%
glibc
"#,
            name, version, reason
        );

        fs::write(pkg_dir.join("desc"), desc_content).unwrap();
    }

    #[test]
    fn test_read_mock_packages() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path();

        // Criar pacotes mock
        create_mock_package(db_path, "vim", "9.0", "0");
        create_mock_package(db_path, "ncurses", "6.4", "1");

        let reader = PacmanReader::with_path(db_path).unwrap();
        let packages = reader.read_all_packages().unwrap();

        assert_eq!(packages.len(), 2);

        let vim = packages.iter().find(|p| p.name == "vim").unwrap();
        assert_eq!(vim.version, "9.0");
        assert!(vim.is_explicit());

        let ncurses = packages.iter().find(|p| p.name == "ncurses").unwrap();
        assert_eq!(ncurses.version, "6.4");
        assert!(ncurses.is_dependency());
    }

    #[test]
    fn test_validate_integrity() {
        let temp_dir = TempDir::new().unwrap();
        create_mock_package(temp_dir.path(), "test", "1.0", "0");

        let reader = PacmanReader::with_path(temp_dir.path()).unwrap();
        assert!(reader.validate_integrity().is_ok());
    }

    #[test]
    fn test_invalid_db_path() {
        let result = PacmanReader::with_path("/nonexistent/path");
        assert!(result.is_err());
    }
}
