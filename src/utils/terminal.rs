//! Utilitários de terminal

use std::io::{self, IsTerminal};

/// Verifica se stdout é um terminal interativo
pub fn is_interactive() -> bool {
    io::stdout().is_terminal()
}

/// Verifica se stderr é um terminal
pub fn is_stderr_terminal() -> bool {
    io::stderr().is_terminal()
}

/// Verifica se cores devem ser desabilitadas (NO_COLOR env)
pub fn should_disable_colors() -> bool {
    std::env::var("NO_COLOR").is_ok()
}

/// Retorna a largura do terminal, ou um padrão se não disponível
pub fn terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

/// Limpa a linha atual do terminal
pub fn clear_line() {
    if is_interactive() {
        print!("\r\x1b[K");
    }
}

/// Move cursor para cima N linhas
pub fn move_up(n: usize) {
    if is_interactive() {
        print!("\x1b[{}A", n);
    }
}
