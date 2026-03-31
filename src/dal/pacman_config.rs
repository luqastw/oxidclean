//! Leitor de configuração do Pacman

use crate::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Caminho padrão do arquivo de configuração do Pacman
pub const PACMAN_CONF_PATH: &str = "/etc/pacman.conf";

/// Configuração do Pacman
#[derive(Debug, Clone)]
pub struct PacmanConfig {
    /// Diretório de cache
    pub cache_dir: PathBuf,

    /// Diretório de logs
    pub log_file: PathBuf,

    /// Repositórios habilitados
    pub repositories: Vec<String>,

    /// Opções gerais
    pub options: HashMap<String, String>,
}

impl Default for PacmanConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("/var/cache/pacman/pkg"),
            log_file: PathBuf::from("/var/log/pacman.log"),
            repositories: vec![
                "core".to_string(),
                "extra".to_string(),
                "multilib".to_string(),
            ],
            options: HashMap::new(),
        }
    }
}

impl PacmanConfig {
    /// Carrega a configuração do arquivo padrão
    pub fn load() -> Result<Self> {
        Self::load_from(PACMAN_CONF_PATH)
    }

    /// Carrega a configuração de um arquivo específico
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        Self::parse(&content)
    }

    /// Parse do conteúdo do arquivo de configuração
    fn parse(content: &str) -> Result<Self> {
        let mut config = Self::default();
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();

            // Ignorar comentários e linhas vazias
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Seção
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();

                // Adicionar repositório
                if current_section != "options" {
                    config.repositories.push(current_section.clone());
                }
                continue;
            }

            // Opção
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "CacheDir" => {
                        config.cache_dir = PathBuf::from(value);
                    }
                    "LogFile" => {
                        config.log_file = PathBuf::from(value);
                    }
                    _ => {
                        if current_section == "options" {
                            config.options.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
        }

        // Remover duplicatas dos repositórios
        config.repositories.sort();
        config.repositories.dedup();

        Ok(config)
    }

    /// Retorna o diretório de cache do pacman
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PacmanConfig::default();
        assert_eq!(config.cache_dir, PathBuf::from("/var/cache/pacman/pkg"));
        assert!(config.repositories.contains(&"core".to_string()));
    }

    #[test]
    fn test_parse_config() {
        let content = r#"
# Pacman config
[options]
CacheDir = /custom/cache
LogFile = /custom/log

[core]
Include = /etc/pacman.d/mirrorlist

[extra]
Include = /etc/pacman.d/mirrorlist
"#;

        let config = PacmanConfig::parse(content).unwrap();
        assert_eq!(config.cache_dir, PathBuf::from("/custom/cache"));
        assert_eq!(config.log_file, PathBuf::from("/custom/log"));
        assert!(config.repositories.contains(&"core".to_string()));
        assert!(config.repositories.contains(&"extra".to_string()));
    }
}
