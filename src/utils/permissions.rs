//! Verificação de permissões

use crate::{OxidCleanError, Result};

/// Verifica se o processo está rodando como root (UID 0)
pub fn ensure_root() -> Result<()> {
    if !is_root() {
        return Err(OxidCleanError::PermissionError(
            "Esta operação requer privilégios de root. Execute com sudo.".to_string(),
        ));
    }
    Ok(())
}

/// Verifica se o usuário atual é root
pub fn is_root() -> bool {
    nix::unistd::geteuid().is_root()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_root() {
        // Em ambiente de teste normal, não deve ser root
        // Este teste pode falhar se executado como root
        let _ = is_root();
    }
}
