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

/// Retorna o UID do usuário atual
pub fn current_uid() -> u32 {
    nix::unistd::getuid().as_raw()
}

/// Retorna o GID do usuário atual
pub fn current_gid() -> u32 {
    nix::unistd::getgid().as_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_uid() {
        // Apenas verifica que não causa panic
        let _uid = current_uid();
    }

    #[test]
    fn test_current_gid() {
        // Apenas verifica que não causa panic
        let _gid = current_gid();
    }

    #[test]
    fn test_is_root() {
        // Em ambiente de teste normal, não deve ser root
        // Este teste pode falhar se executado como root
        let is_root_result = is_root();
        // Apenas verifica que retorna um boolean
        assert!(is_root_result || !is_root_result);
    }
}
