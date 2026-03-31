//! Modelos de relatório

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Relatório completo de análise do sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemReport {
    /// Total de pacotes instalados
    pub total_packages: usize,

    /// Pacotes órfãos encontrados
    pub orphans: Vec<OrphanPackage>,

    /// Espaço total recuperável (bytes)
    pub recoverable_space: u64,

    /// Estatísticas do cache do pacman
    pub cache_stats: Option<CacheStats>,

    /// Data/hora do scan
    pub scan_timestamp: DateTime<Utc>,
}

impl SystemReport {
    /// Cria um novo relatório vazio
    pub fn new() -> Self {
        Self {
            total_packages: 0,
            orphans: Vec::new(),
            recoverable_space: 0,
            cache_stats: None,
            scan_timestamp: Utc::now(),
        }
    }

    /// Exporta relatório para JSON
    pub fn to_json(&self) -> crate::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Exporta relatório para Markdown
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# OxidClean - Relatório do Sistema\n\n");
        md.push_str(&format!(
            "**Data do Scan:** {}\n\n",
            self.scan_timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        md.push_str("## Estatísticas\n\n");
        md.push_str(&format!(
            "- **Total de pacotes:** {}\n",
            self.total_packages
        ));
        md.push_str(&format!("- **Pacotes órfãos:** {}\n", self.orphans.len()));
        md.push_str(&format!(
            "- **Espaço recuperável:** {}\n\n",
            crate::utils::humanize_bytes(self.recoverable_space)
        ));

        if !self.orphans.is_empty() {
            md.push_str("## Pacotes Órfãos\n\n");
            md.push_str("| Nome | Versão | Tamanho | Risco |\n");
            md.push_str("|------|--------|---------|-------|\n");

            for orphan in &self.orphans {
                let risk = match orphan.risk_level {
                    RiskLevel::Safe => "Seguro",
                    RiskLevel::Caution => "Atenção",
                    RiskLevel::Critical => "Crítico",
                };
                md.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    orphan.name,
                    orphan.version,
                    crate::utils::humanize_bytes(orphan.size),
                    risk
                ));
            }
        }

        md
    }
}

impl Default for SystemReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Informações de um pacote órfão
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanPackage {
    /// Nome do pacote
    pub name: String,

    /// Versão instalada
    pub version: String,

    /// Tamanho em disco (bytes)
    pub size: u64,

    /// Data de instalação
    pub install_date: Option<DateTime<Utc>>,

    /// Nível de risco para remoção
    pub risk_level: RiskLevel,

    /// Descrição do pacote
    pub description: Option<String>,
}

impl OrphanPackage {
    /// Cria um novo OrphanPackage a partir de um Package
    pub fn from_package(pkg: &crate::models::Package, risk_level: RiskLevel) -> Self {
        Self {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            size: pkg.size,
            install_date: pkg.install_date,
            risk_level,
            description: pkg.description.clone(),
        }
    }
}

/// Nível de risco para remoção de um pacote
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Seguro para remover (verde)
    Safe,

    /// Atenção necessária (amarelo)
    Caution,

    /// Crítico - não remover (vermelho)
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        Self::Safe
    }
}

/// Estatísticas do cache do pacman
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Número total de pacotes em cache
    pub total_packages: usize,

    /// Espaço total ocupado pelo cache (bytes)
    pub total_size: u64,

    /// Número de pacotes não mais instalados
    pub unused_packages: usize,

    /// Espaço ocupado por pacotes não instalados (bytes)
    pub unused_size: u64,
}

impl CacheStats {
    /// Cria estatísticas vazias
    pub fn new() -> Self {
        Self {
            total_packages: 0,
            total_size: 0,
            unused_packages: 0,
            unused_size: 0,
        }
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_report() {
        let report = SystemReport::new();
        assert_eq!(report.total_packages, 0);
        assert!(report.orphans.is_empty());
        assert_eq!(report.recoverable_space, 0);
    }

    #[test]
    fn test_to_json() {
        let report = SystemReport::new();
        let json = report.to_json().unwrap();
        assert!(json.contains("total_packages"));
        assert!(json.contains("orphans"));
    }

    #[test]
    fn test_to_markdown() {
        let mut report = SystemReport::new();
        report.total_packages = 100;
        report.orphans.push(OrphanPackage {
            name: "test-pkg".to_string(),
            version: "1.0".to_string(),
            size: 1024,
            install_date: None,
            risk_level: RiskLevel::Safe,
            description: None,
        });

        let md = report.to_markdown();
        assert!(md.contains("OxidClean"));
        assert!(md.contains("test-pkg"));
        assert!(md.contains("100"));
    }
}
