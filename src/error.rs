//! Tipos de erro customizados para OxidClean

use std::path::PathBuf;
use thiserror::Error;

/// Tipo Result customizado para OxidClean
pub type Result<T> = std::result::Result<T, OxidCleanError>;

/// Erros possíveis durante execução do OxidClean
#[derive(Error, Debug)]
pub enum OxidCleanError {
    /// Banco de dados do Pacman não encontrado ou corrompido
    #[error("Banco de dados do Pacman não encontrado ou corrompido em: {0}")]
    PacmanDbNotFound(PathBuf),

    /// Erro ao ler banco de dados do Pacman
    #[error("Erro ao ler banco de dados do Pacman: {0}")]
    PacmanDbReadError(String),

    /// Permissões insuficientes
    #[error("Permissões insuficientes: {0}")]
    PermissionError(String),

    /// Pacote não encontrado
    #[error("Pacote '{0}' não encontrado")]
    PackageNotFound(String),

    /// Erro ao parsear configuração
    #[error("Erro ao parsear configuração: {0}")]
    ConfigError(String),

    /// Erro de IO
    #[error("Erro de IO: {0}")]
    IoError(#[from] std::io::Error),

    /// Sistema não é Arch Linux ou derivado
    #[error("Sistema não suportado: esperado Arch Linux ou derivado, encontrado: {0}")]
    UnsupportedSystem(String),

    /// Operação cancelada pelo usuário
    #[error("Operação cancelada pelo usuário")]
    OperationCancelled,

    /// Erro ao executar comando do pacman
    #[error("Erro ao executar pacman: {0}")]
    PacmanExecError(String),

    /// Pacote protegido não pode ser removido
    #[error("Pacote '{0}' está protegido e não pode ser removido")]
    ProtectedPackage(String),

    /// Erro de serialização/deserialização
    #[error("Erro de serialização: {0}")]
    SerializationError(String),

    /// Erro ao parsear arquivo desc do pacman
    #[error("Erro ao parsear arquivo desc do pacote '{0}': {1}")]
    PackageParseError(String, String),
}

impl From<serde_json::Error> for OxidCleanError {
    fn from(err: serde_json::Error) -> Self {
        OxidCleanError::SerializationError(err.to_string())
    }
}

impl From<toml::de::Error> for OxidCleanError {
    fn from(err: toml::de::Error) -> Self {
        OxidCleanError::ConfigError(err.to_string())
    }
}
