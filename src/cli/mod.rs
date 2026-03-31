//! # Módulo CLI
//!
//! Interface de linha de comando usando [clap](https://docs.rs/clap).
//!
//! ## Comandos disponíveis
//!
//! - `scan` - Escaneia o sistema e exibe relatório de pacotes órfãos
//! - `clean` - Remove pacotes órfãos de forma interativa
//! - `analyze <pacote>` - Analisa dependências de um pacote específico
//! - `cache` - Gerencia o cache do pacman
//! - `list` - Lista todos os pacotes órfãos detectados
//! - `completion <shell>` - Gera scripts de auto-completar
//!
//! ## Flags globais
//!
//! - `-v, --verbose` - Modo verboso
//! - `-q, --quiet` - Modo silencioso

pub mod commands;
pub mod output;

pub use commands::{Cli, Commands, Shell};
