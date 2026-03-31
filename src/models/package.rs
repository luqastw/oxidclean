//! Modelo de pacote

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Representa um pacote instalado no sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Nome do pacote
    pub name: String,

    /// Versão instalada
    pub version: String,

    /// Tamanho em disco (bytes)
    pub size: u64,

    /// Tipo de instalação
    pub install_reason: InstallReason,

    /// Lista de dependências diretas
    pub dependencies: Vec<String>,

    /// Lista de dependências opcionais
    pub optional_deps: Vec<String>,

    /// Data de instalação
    pub install_date: Option<DateTime<Utc>>,

    /// Descrição do pacote
    pub description: Option<String>,

    /// Arquitetura do pacote
    pub arch: Option<String>,

    /// URL do pacote
    pub url: Option<String>,
}

impl Package {
    /// Cria um novo pacote com valores mínimos
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            size: 0,
            install_reason: InstallReason::Explicit,
            dependencies: Vec::new(),
            optional_deps: Vec::new(),
            install_date: None,
            description: None,
            arch: None,
            url: None,
        }
    }

    /// Verifica se o pacote foi instalado explicitamente
    pub fn is_explicit(&self) -> bool {
        self.install_reason == InstallReason::Explicit
    }

    /// Verifica se o pacote é uma dependência
    pub fn is_dependency(&self) -> bool {
        self.install_reason == InstallReason::Dependency
    }
}

/// Razão da instalação do pacote
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallReason {
    /// Pacote instalado explicitamente pelo usuário
    Explicit,

    /// Pacote instalado como dependência de outro
    Dependency,
}

impl Default for InstallReason {
    fn default() -> Self {
        Self::Explicit
    }
}

impl std::fmt::Display for InstallReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallReason::Explicit => write!(f, "explícito"),
            InstallReason::Dependency => write!(f, "dependência"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_package() {
        let pkg = Package::new("vim".to_string(), "9.0.0".to_string());
        assert_eq!(pkg.name, "vim");
        assert_eq!(pkg.version, "9.0.0");
        assert!(pkg.is_explicit());
    }

    #[test]
    fn test_install_reason() {
        let mut pkg = Package::new("test".to_string(), "1.0".to_string());
        assert!(pkg.is_explicit());
        assert!(!pkg.is_dependency());

        pkg.install_reason = InstallReason::Dependency;
        assert!(!pkg.is_explicit());
        assert!(pkg.is_dependency());
    }
}
