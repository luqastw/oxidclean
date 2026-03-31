//! Leitor de configuração da aplicação

use crate::models::Config;
use crate::Result;
use std::fs;
use std::path::Path;

/// Leitor de configuração do OxidClean
pub struct ConfigReader;

impl ConfigReader {
    /// Carrega a configuração do arquivo padrão ou retorna configuração padrão
    pub fn load() -> Config {
        Self::load_from(&Config::config_path()).unwrap_or_default()
    }

    /// Carrega a configuração de um arquivo específico
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config> {
        let content = fs::read_to_string(path.as_ref())?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Salva a configuração no arquivo padrão
    pub fn save(config: &Config) -> Result<()> {
        Self::save_to(config, &Config::config_path())
    }

    /// Salva a configuração em um arquivo específico
    pub fn save_to<P: AsRef<Path>>(config: &Config, path: P) -> Result<()> {
        let path = path.as_ref();

        // Criar diretório se necessário
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(config)
            .map_err(|e| crate::OxidCleanError::ConfigError(e.to_string()))?;

        fs::write(path, content)?;
        Ok(())
    }

    /// Verifica se o arquivo de configuração existe
    pub fn exists() -> bool {
        Config::config_path().exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_default() {
        let config = ConfigReader::load();
        assert!(config.protected_packages.contains(&"base".to_string()));
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let mut config = Config::default();
        config.ignored_packages = vec!["test-pkg".to_string()];

        ConfigReader::save_to(&config, &config_path).unwrap();

        let loaded = ConfigReader::load_from(&config_path).unwrap();
        assert!(loaded.ignored_packages.contains(&"test-pkg".to_string()));
    }
}
