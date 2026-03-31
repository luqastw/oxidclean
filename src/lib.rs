//! # OxidClean
//!
//! Gerenciador de pacotes órfãos para Arch Linux e derivados.
//!
//! OxidClean é uma ferramenta de terminal escrita em Rust que identifica e remove
//! pacotes instalados como dependências que não são mais necessários, otimizando
//! o uso de espaço em disco.
//!
//! ## Funcionalidades
//!
//! - **Detecção de órfãos**: Identifica pacotes instalados como dependência que
//!   não são mais requeridos por nenhum outro pacote
//! - **Análise de dependências**: Grafo completo de dependências com detecção de ciclos
//! - **Limpeza de cache**: Gerenciamento do cache do pacman
//! - **Classificação de risco**: Pacotes são categorizados como Safe, Caution ou Critical
//! - **Proteção de pacotes**: Pacotes críticos do sistema são protegidos contra remoção
//!
//! ## Exemplo de uso
//!
//! ```no_run
//! use oxidclean::core::Scanner;
//!
//! fn main() -> oxidclean::Result<()> {
//!     // Criar scanner e executar análise
//!     let mut scanner = Scanner::new()?;
//!     let report = scanner.scan()?;
//!     
//!     println!("Pacotes instalados: {}", report.total_packages);
//!     println!("Pacotes órfãos: {}", report.orphans.len());
//!     println!("Espaço recuperável: {} bytes", report.recoverable_space);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Módulos
//!
//! - [`cli`]: Interface de linha de comando (clap)
//! - [`core`]: Lógica principal (Scanner, Analyzer, Cleaner, CacheManager)
//! - [`dal`]: Camada de acesso a dados (PacmanReader, ConfigReader)
//! - [`models`]: Estruturas de dados (Package, DependencyGraph, Report)
//! - [`utils`] - Utilitários (formatação, permissões, progresso)
//! - [`error`]: Tipos de erro customizados

pub mod cli;
pub mod core;
pub mod dal;
pub mod error;
pub mod models;
pub mod utils;

pub use error::{OxidCleanError, Result};
