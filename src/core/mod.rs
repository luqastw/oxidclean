//! Módulo Core - Lógica principal da aplicação

pub mod analyzer;
pub mod cache_manager;
pub mod cleaner;
pub mod scanner;
pub mod validator;

pub use analyzer::Analyzer;
pub use cleaner::Cleaner;
pub use scanner::Scanner;
pub use validator::Validator;
