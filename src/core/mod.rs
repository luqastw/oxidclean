//! # Módulo Core
//!
//! Contém a lógica principal da aplicação OxidClean.
//!
//! ## Componentes
//!
//! - [`Scanner`]: Escaneia o sistema e identifica pacotes órfãos
//! - [`Analyzer`]: Analisa dependências de pacotes específicos
//! - [`Cleaner`]: Remove pacotes de forma segura e interativa
//! - [`Validator`]: Valida o estado do sistema (Arch Linux)
//!
//! ## Exemplo
//!
//! ```no_run
//! use oxidclean::core::{Scanner, Analyzer, Cleaner};
//!
//! // Escanear sistema
//! let mut scanner = Scanner::new().unwrap();
//! let report = scanner.scan().unwrap();
//!
//! // Analisar pacote específico
//! let analyzer = Analyzer::new().unwrap();
//! let analysis = analyzer.analyze_package("vim").unwrap();
//! ```

pub mod analyzer;
pub mod cache_manager;
pub mod cleaner;
pub mod scanner;
pub mod validator;

pub use analyzer::Analyzer;
pub use cleaner::Cleaner;
pub use scanner::Scanner;
pub use validator::Validator;
