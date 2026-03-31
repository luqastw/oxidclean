//! Módulo DAL - Data Access Layer

pub mod config;
pub mod logger;
pub mod pacman_config;
pub mod pacman_db;

pub use config::ConfigReader;
pub use logger::OperationLogger;
pub use pacman_db::PacmanReader;
