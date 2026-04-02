//! Detecção de informações do sistema

use crate::{OxidCleanError, Result};
use std::fs;
use std::path::Path;

/// Distribuições suportadas
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Distro {
    /// Arch Linux vanilla
    Arch,
    /// Manjaro
    Manjaro,
    /// EndeavourOS
    EndeavourOS,
    /// Garuda Linux
    Garuda,
    /// ArcoLinux
    ArcoLinux,
    /// Artix Linux
    Artix,
    /// Outra distribuição baseada em Arch
    ArchBased(String),
    /// Distribuição desconhecida/não suportada
    Unknown(String),
}

impl Distro {
    /// Verifica se a distribuição é baseada em Arch
    pub fn is_arch_based(&self) -> bool {
        matches!(
            self,
            Distro::Arch
                | Distro::Manjaro
                | Distro::EndeavourOS
                | Distro::Garuda
                | Distro::ArcoLinux
                | Distro::Artix
                | Distro::ArchBased(_)
        )
    }
}

impl std::fmt::Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distro::Arch => write!(f, "Arch Linux"),
            Distro::Manjaro => write!(f, "Manjaro"),
            Distro::EndeavourOS => write!(f, "EndeavourOS"),
            Distro::Garuda => write!(f, "Garuda Linux"),
            Distro::ArcoLinux => write!(f, "ArcoLinux"),
            Distro::Artix => write!(f, "Artix Linux"),
            Distro::ArchBased(name) => write!(f, "{} (Arch-based)", name),
            Distro::Unknown(name) => write!(f, "{} (não suportado)", name),
        }
    }
}

/// Detecta a distribuição Linux atual
pub fn detect_distro() -> Result<Distro> {
    let os_release_path = Path::new("/etc/os-release");

    if !os_release_path.exists() {
        return Err(OxidCleanError::UnsupportedSystem(
            "Arquivo /etc/os-release não encontrado".to_string(),
        ));
    }

    let content = fs::read_to_string(os_release_path)?;

    let mut id = String::new();
    let mut id_like = String::new();
    let mut name = String::new();

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("ID=") {
            id = value.trim_matches('"').to_lowercase();
        } else if let Some(value) = line.strip_prefix("ID_LIKE=") {
            id_like = value.trim_matches('"').to_lowercase();
        } else if let Some(value) = line.strip_prefix("NAME=") {
            name = value.trim_matches('"').to_string();
        }
    }

    let distro = match id.as_str() {
        "arch" => Distro::Arch,
        "manjaro" => Distro::Manjaro,
        "endeavouros" => Distro::EndeavourOS,
        "garuda" => Distro::Garuda,
        "arcolinux" => Distro::ArcoLinux,
        "artix" => Distro::Artix,
        _ => {
            // Verificar se é baseado em Arch
            if id_like.contains("arch") || has_pacman() {
                Distro::ArchBased(name.clone())
            } else {
                Distro::Unknown(name.clone())
            }
        }
    };

    Ok(distro)
}

/// Verifica se o sistema tem pacman instalado
fn has_pacman() -> bool {
    Path::new("/usr/bin/pacman").exists()
}

/// Verifica se o sistema é suportado (Arch-based)
pub fn check_system_support() -> Result<Distro> {
    let distro = detect_distro()?;

    if !distro.is_arch_based() {
        return Err(OxidCleanError::UnsupportedSystem(format!(
            "Sistema '{}' não é baseado em Arch Linux",
            distro
        )));
    }

    // Verificar se pacman está disponível
    if !has_pacman() {
        return Err(OxidCleanError::UnsupportedSystem(
            "Pacman não encontrado em /usr/bin/pacman".to_string(),
        ));
    }

    Ok(distro)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_display() {
        assert_eq!(format!("{}", Distro::Arch), "Arch Linux");
        assert_eq!(format!("{}", Distro::Manjaro), "Manjaro");
        assert_eq!(
            format!("{}", Distro::ArchBased("Custom".to_string())),
            "Custom (Arch-based)"
        );
    }

    #[test]
    fn test_is_arch_based() {
        assert!(Distro::Arch.is_arch_based());
        assert!(Distro::Manjaro.is_arch_based());
        assert!(Distro::ArchBased("Custom".to_string()).is_arch_based());
        assert!(!Distro::Unknown("Ubuntu".to_string()).is_arch_based());
    }
}
