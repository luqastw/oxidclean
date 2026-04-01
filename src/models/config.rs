//! Modelo de configuração

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuração da aplicação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Pacotes que devem ser ignorados na análise
    #[serde(default)]
    pub ignored_packages: Vec<String>,

    /// Pacotes protegidos que nunca devem ser removidos
    #[serde(default = "default_protected_packages")]
    pub protected_packages: Vec<String>,

    /// Modo padrão interativo
    #[serde(default = "default_interactive")]
    pub interactive: bool,

    /// Número de versões de cache para manter
    #[serde(default = "default_cache_versions")]
    pub cache_keep_versions: usize,

    /// Caminho do arquivo de log
    #[serde(default = "default_log_path")]
    pub log_path: PathBuf,
}

fn default_protected_packages() -> Vec<String> {
    vec![
        "base".to_string(),
        "linux".to_string(),
        "linux-lts".to_string(),
        "linux-zen".to_string(),
        "linux-hardened".to_string(),
        "linux-firmware".to_string(),
        "pacman".to_string(),
        "systemd".to_string(),
        "glibc".to_string(),
        "coreutils".to_string(),
        "bash".to_string(),
        "shadow".to_string(),
        "util-linux".to_string(),
    ]
}

fn default_interactive() -> bool {
    true
}

fn default_cache_versions() -> usize {
    3
}

fn default_log_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/root"))
        .join(".config")
        .join("oxidclean")
        .join("history.log")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ignored_packages: Vec::new(),
            protected_packages: default_protected_packages(),
            interactive: default_interactive(),
            cache_keep_versions: default_cache_versions(),
            log_path: default_log_path(),
        }
    }
}

impl Config {
    /// Caminho padrão do arquivo de configuração
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/root"))
            .join(".config")
            .join("oxidclean")
            .join("config.toml")
    }

    /// Verifica se um pacote está na lista de ignorados
    pub fn is_ignored(&self, package: &str) -> bool {
        self.ignored_packages.contains(&package.to_string())
    }

    /// Verifica se um pacote está protegido
    pub fn is_protected(&self, package: &str) -> bool {
        self.protected_packages.contains(&package.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert!(config.protected_packages.contains(&"base".to_string()));
        assert!(config.protected_packages.contains(&"linux".to_string()));
        assert!(config.protected_packages.contains(&"pacman".to_string()));
        assert!(config.interactive);
        assert_eq!(config.cache_keep_versions, 3);
    }

    #[test]
    fn test_is_protected() {
        let config = Config::default();

        assert!(config.is_protected("base"));
        assert!(config.is_protected("pacman"));
        assert!(!config.is_protected("vim"));
    }

    #[test]
    fn test_is_ignored() {
        let mut config = Config::default();
        config.ignored_packages = vec!["test-pkg".to_string()];

        assert!(config.is_ignored("test-pkg"));
        assert!(!config.is_ignored("other-pkg"));
    }
}
