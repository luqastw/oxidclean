//! Módulo Utils - Utilitários

pub mod disk_usage;
pub mod permissions;
pub mod system_info;
pub mod terminal;

pub use disk_usage::{calculate_size, humanize_bytes};
pub use permissions::ensure_root;
pub use system_info::{detect_distro, Distro};
