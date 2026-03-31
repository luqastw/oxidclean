//! Logger de operações

use crate::models::Package;
use crate::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Tipo de operação registrada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Remoção de pacote
    Remove,
    /// Limpeza de cache
    CacheClean,
    /// Scan do sistema
    Scan,
}

/// Entrada de log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp da operação
    pub timestamp: String,

    /// Tipo de operação
    pub operation: OperationType,

    /// Nome do pacote (se aplicável)
    pub package_name: Option<String>,

    /// Versão do pacote (se aplicável)
    pub package_version: Option<String>,

    /// Tamanho liberado em bytes (se aplicável)
    pub size_freed: Option<u64>,

    /// Detalhes adicionais
    pub details: Option<String>,

    /// Sucesso da operação
    pub success: bool,
}

/// Logger de operações do OxidClean
pub struct OperationLogger {
    log_path: PathBuf,
}

impl OperationLogger {
    /// Cria um novo logger
    pub fn new() -> Result<Self> {
        let log_path = Self::default_log_path();
        Self::with_path(log_path)
    }

    /// Cria um logger com caminho customizado
    pub fn with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let log_path = path.as_ref().to_path_buf();

        // Criar diretório se necessário
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self { log_path })
    }

    /// Caminho padrão do log
    fn default_log_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("oxidclean")
            .join("history.log")
    }

    /// Registra a remoção de um pacote
    pub fn log_removal(&self, package: &Package, success: bool) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            operation: OperationType::Remove,
            package_name: Some(package.name.clone()),
            package_version: Some(package.version.clone()),
            size_freed: Some(package.size),
            details: None,
            success,
        };

        self.write_entry(&entry)
    }

    /// Registra limpeza de cache
    pub fn log_cache_clean(&self, size_freed: u64, success: bool) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            operation: OperationType::CacheClean,
            package_name: None,
            package_version: None,
            size_freed: Some(size_freed),
            details: None,
            success,
        };

        self.write_entry(&entry)
    }

    /// Registra um scan do sistema
    pub fn log_scan(&self, orphans_found: usize, success: bool) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            operation: OperationType::Scan,
            package_name: None,
            package_version: None,
            size_freed: None,
            details: Some(format!("{} órfãos encontrados", orphans_found)),
            success,
        };

        self.write_entry(&entry)
    }

    /// Registra uma operação genérica
    pub fn log_operation(
        &self,
        operation: OperationType,
        details: &str,
        success: bool,
    ) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            operation,
            package_name: None,
            package_version: None,
            size_freed: None,
            details: Some(details.to_string()),
            success,
        };

        self.write_entry(&entry)
    }

    /// Escreve uma entrada no log
    fn write_entry(&self, entry: &LogEntry) -> Result<()> {
        let json = serde_json::to_string(entry)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        writeln!(file, "{}", json)?;
        Ok(())
    }

    /// Lê todas as entradas do log
    pub fn read_all(&self) -> Result<Vec<LogEntry>> {
        if !self.log_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.log_path)?;
        let entries: Vec<LogEntry> = content
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(entries)
    }

    /// Retorna o caminho do arquivo de log
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }
}

impl Default for OperationLogger {
    fn default() -> Self {
        Self::new().expect("Falha ao criar OperationLogger")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_log_removal() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test.log");

        let logger = OperationLogger::with_path(&log_path).unwrap();
        let pkg = Package::new("test-pkg".to_string(), "1.0".to_string());

        logger.log_removal(&pkg, true).unwrap();

        let entries = logger.read_all().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].package_name, Some("test-pkg".to_string()));
    }

    #[test]
    fn test_log_scan() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("test.log");

        let logger = OperationLogger::with_path(&log_path).unwrap();
        logger.log_scan(5, true).unwrap();

        let entries = logger.read_all().unwrap();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].details.as_ref().unwrap().contains("5"));
    }
}
