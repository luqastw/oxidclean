//! Scanner de pacotes do sistema

use crate::dal::{ConfigReader, OperationLogger, PacmanReader};
use crate::models::{Config, DependencyGraph, OrphanPackage, Package, RiskLevel, SystemReport};
use crate::Result;
use chrono::Utc;
use log::{debug, info};
use std::time::Instant;

/// Scanner de pacotes do sistema
pub struct Scanner {
    reader: PacmanReader,
    config: Config,
    cache: Option<ScanCache>,
}

impl Scanner {
    /// Cria um novo scanner
    pub fn new() -> Result<Self> {
        Ok(Self {
            reader: PacmanReader::new()?,
            config: ConfigReader::load(),
            cache: None,
        })
    }

    /// Cria um scanner com reader customizado (útil para testes)
    pub fn with_reader(reader: PacmanReader) -> Self {
        Self {
            reader,
            config: ConfigReader::load(),
            cache: None,
        }
    }

    /// Executa um scan completo do sistema
    pub fn scan(&mut self) -> Result<SystemReport> {
        // Verificar cache
        if let Some(ref cache) = self.cache {
            if cache.is_valid() {
                info!(
                    "Usando cache de scan (válido por mais {} segundos)",
                    cache.remaining_seconds()
                );
                return Ok(cache.report.clone());
            }
        }

        let start = Instant::now();
        info!("Iniciando scan do sistema...");

        // 1. Validar integridade do DB
        self.reader.validate_integrity()?;
        debug!("Banco de dados validado");

        // 2. Ler todos os pacotes
        let packages = self.reader.read_all_packages()?;
        info!("Encontrados {} pacotes instalados", packages.len());

        // 3. Construir grafo de dependências
        let graph = DependencyGraph::build(&packages);
        debug!("Grafo de dependências construído");

        // 4. Identificar órfãos
        let orphan_names = graph.find_orphans();
        info!("Encontrados {} pacotes órfãos", orphan_names.len());

        // 5. Construir lista de órfãos com detalhes
        let orphans = self.build_orphan_list(&packages, &orphan_names);

        // 6. Calcular espaço recuperável
        let recoverable_space: u64 = orphans.iter().map(|o| o.size).sum();

        // 7. Gerar relatório
        let report = SystemReport {
            total_packages: packages.len(),
            orphans,
            recoverable_space,
            cache_stats: None, // TODO: Implementar cache stats
            scan_timestamp: Utc::now(),
        };

        let elapsed = start.elapsed();
        info!("Scan completo em {:.2}s", elapsed.as_secs_f64());

        // 8. Cachear resultado
        self.cache = Some(ScanCache::new(report.clone()));

        // 9. Log da operação
        if let Ok(logger) = OperationLogger::new() {
            let _ = logger.log_scan(report.orphans.len(), true);
        }

        Ok(report)
    }

    /// Constrói lista de pacotes órfãos com detalhes
    fn build_orphan_list(
        &self,
        packages: &[Package],
        orphan_names: &[String],
    ) -> Vec<OrphanPackage> {
        let mut orphans = Vec::new();

        for name in orphan_names {
            // Pular pacotes ignorados
            if self.config.is_ignored(name) {
                debug!("Pacote '{}' ignorado pela configuração", name);
                continue;
            }

            // Encontrar pacote
            if let Some(pkg) = packages.iter().find(|p| &p.name == name) {
                let risk_level = self.classify_risk(pkg);
                orphans.push(OrphanPackage::from_package(pkg, risk_level));
            }
        }

        orphans
    }

    /// Classifica o nível de risco de um pacote
    fn classify_risk(&self, pkg: &Package) -> RiskLevel {
        // Pacotes protegidos são críticos
        if self.config.is_protected(&pkg.name) {
            return RiskLevel::Critical;
        }

        // Pacotes relacionados a sistema/kernel merecem atenção
        let caution_patterns = [
            "linux", "kernel", "nvidia", "amd", "mesa", "xorg", "wayland", "systemd", "dbus",
            "polkit", "sudo", "openssh",
        ];

        for pattern in &caution_patterns {
            if pkg.name.contains(pattern) {
                return RiskLevel::Caution;
            }
        }

        // Libs geralmente são seguras de remover se órfãs
        RiskLevel::Safe
    }

    /// Invalida o cache forçando um novo scan
    pub fn invalidate_cache(&mut self) {
        self.cache = None;
    }

    /// Verifica se há cache válido
    pub fn has_valid_cache(&self) -> bool {
        self.cache.as_ref().is_some_and(|c| c.is_valid())
    }
}

/// Cache de resultados do scan
#[derive(Clone)]
struct ScanCache {
    report: SystemReport,
    created_at: Instant,
}

impl ScanCache {
    /// TTL do cache em segundos (5 minutos)
    const TTL_SECONDS: u64 = 300;

    fn new(report: SystemReport) -> Self {
        Self {
            report,
            created_at: Instant::now(),
        }
    }

    fn is_valid(&self) -> bool {
        self.created_at.elapsed().as_secs() < Self::TTL_SECONDS
    }

    fn remaining_seconds(&self) -> u64 {
        Self::TTL_SECONDS.saturating_sub(self.created_at.elapsed().as_secs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_mock_db(dir: &std::path::Path) {
        // Criar pacote explícito
        let vim_dir = dir.join("vim-9.0");
        fs::create_dir_all(&vim_dir).unwrap();
        fs::write(
            vim_dir.join("desc"),
            r#"%NAME%
vim

%VERSION%
9.0

%ISIZE%
2048

%REASON%
0

%DEPENDS%
ncurses
"#,
        )
        .unwrap();

        // Criar dependência usada
        let ncurses_dir = dir.join("ncurses-6.4");
        fs::create_dir_all(&ncurses_dir).unwrap();
        fs::write(
            ncurses_dir.join("desc"),
            r#"%NAME%
ncurses

%VERSION%
6.4

%ISIZE%
1024

%REASON%
1
"#,
        )
        .unwrap();

        // Criar pacote órfão
        let orphan_dir = dir.join("old-lib-1.0");
        fs::create_dir_all(&orphan_dir).unwrap();
        fs::write(
            orphan_dir.join("desc"),
            r#"%NAME%
old-lib

%VERSION%
1.0

%ISIZE%
512

%REASON%
1
"#,
        )
        .unwrap();
    }

    #[test]
    fn test_scan() {
        let temp_dir = TempDir::new().unwrap();
        create_mock_db(temp_dir.path());

        let reader = PacmanReader::with_path(temp_dir.path()).unwrap();
        let mut scanner = Scanner::with_reader(reader);

        let report = scanner.scan().unwrap();

        assert_eq!(report.total_packages, 3);
        assert_eq!(report.orphans.len(), 1);
        assert_eq!(report.orphans[0].name, "old-lib");
    }

    #[test]
    fn test_cache() {
        let temp_dir = TempDir::new().unwrap();
        create_mock_db(temp_dir.path());

        let reader = PacmanReader::with_path(temp_dir.path()).unwrap();
        let mut scanner = Scanner::with_reader(reader);

        // Primeiro scan
        let _report1 = scanner.scan().unwrap();
        assert!(scanner.has_valid_cache());

        // Segundo scan deve usar cache
        let _report2 = scanner.scan().unwrap();
        assert!(scanner.has_valid_cache());

        // Invalidar cache
        scanner.invalidate_cache();
        assert!(!scanner.has_valid_cache());
    }
}
