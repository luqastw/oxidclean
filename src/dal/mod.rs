//! # Módulo DAL (Data Access Layer)
//!
//! Camada de acesso a dados do OxidClean.
//!
//! ## Componentes
//!
//! - [`PacmanReader`]: Lê o banco de dados do pacman (`/var/lib/pacman/local`)
//! - [`ConfigReader`]: Carrega configurações do usuário
//! - [`OperationLogger`]: Registra operações em arquivo de log
//!
//! ## Arquivos acessados
//!
//! - `/var/lib/pacman/local/` - Base de dados de pacotes instalados
//! - `/etc/pacman.conf` - Configuração do pacman
//! - `~/.config/oxidclean/config.toml` - Configuração do usuário
//! - `~/.config/oxidclean/history.log` - Log de operações

pub mod config;
pub mod logger;
pub mod pacman_config;
pub mod pacman_db;

pub use config::ConfigReader;
pub use logger::OperationLogger;
pub use pacman_db::PacmanReader;
