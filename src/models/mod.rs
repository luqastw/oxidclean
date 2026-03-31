//! Módulo Models - Estruturas de dados

pub mod config;
pub mod dependency;
pub mod package;
pub mod report;

pub use config::Config;
pub use dependency::DependencyGraph;
pub use package::{InstallReason, Package};
pub use report::{CacheStats, OrphanPackage, RiskLevel, SystemReport};
