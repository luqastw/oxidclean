//! OxidClean - Gerenciador de pacotes órfãos do Arch Linux
//!
//! Uma ferramenta de terminal feita em Rust para gerenciar pacotes do Arch Linux,
//! servindo como otimizador de dependências e removedor de pacotes que não são
//! utilizados e apenas estão ocupando espaço.

pub mod cli;
pub mod core;
pub mod dal;
pub mod error;
pub mod models;
pub mod utils;

pub use error::{OxidCleanError, Result};
