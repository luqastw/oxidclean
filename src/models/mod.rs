//! # Módulo Models
//!
//! Estruturas de dados utilizadas pelo OxidClean.
//!
//! ## Tipos principais
//!
//! - [`Package`]: Representa um pacote instalado no sistema
//! - [`DependencyGraph`]: Grafo bidirecional de dependências
//! - [`SystemReport`]: Relatório completo de análise do sistema
//! - [`Config`]: Configuração da aplicação
//!
//! ## Enums
//!
//! - [`InstallReason`]: Razão da instalação (Explicit/Dependency)
//! - [`RiskLevel`]: Nível de risco para remoção (Safe/Caution/Critical)

pub mod config;
pub mod dependency;
pub mod package;
pub mod report;

pub use config::Config;
pub use dependency::DependencyGraph;
pub use package::{InstallReason, Package};
pub use report::{CacheStats, OrphanPackage, RiskLevel, SystemReport};
