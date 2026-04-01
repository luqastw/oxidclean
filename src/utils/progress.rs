//! Indicadores de progresso usando indicatif

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Cria uma barra de progresso para operações de limpeza
pub fn create_clean_progress(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Símbolos de status para output
pub mod symbols {
    /// Sucesso
    pub const SUCCESS: &str = "✓";
    /// Aviso/skip
    pub const WARNING: &str = "⚠";
    /// Erro
    pub const ERROR: &str = "✗";
    /// Item de lista
    pub const BULLET: &str = "•";
    /// Seta
    pub const ARROW: &str = "→";
}
