//! Utilitários de cálculo de uso de disco

use std::path::Path;
use walkdir::WalkDir;

/// Calcula o tamanho total de um diretório ou arquivo
pub fn calculate_size(path: &Path) -> u64 {
    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }

    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

/// Converte bytes para formato legível por humanos
pub fn humanize_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Converte string de tamanho do pacman para bytes
/// Exemplo: "1.5 MiB" -> 1572864
pub fn parse_size(size_str: &str) -> u64 {
    let parts: Vec<&str> = size_str.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return 0;
    }

    let value: f64 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let multiplier: u64 = match parts[1].to_uppercase().as_str() {
        "B" => 1,
        "KIB" | "KB" | "K" => 1024,
        "MIB" | "MB" | "M" => 1024 * 1024,
        "GIB" | "GB" | "G" => 1024 * 1024 * 1024,
        "TIB" | "TB" | "T" => 1024 * 1024 * 1024 * 1024,
        _ => 1,
    };

    (value * multiplier as f64) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanize_bytes() {
        assert_eq!(humanize_bytes(0), "0 B");
        assert_eq!(humanize_bytes(512), "512 B");
        assert_eq!(humanize_bytes(1024), "1.00 KB");
        assert_eq!(humanize_bytes(1536), "1.50 KB");
        assert_eq!(humanize_bytes(1048576), "1.00 MB");
        assert_eq!(humanize_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("1024 B"), 1024);
        assert_eq!(parse_size("1 KiB"), 1024);
        assert_eq!(parse_size("1.5 MiB"), 1572864);
        assert_eq!(parse_size("1 GiB"), 1073741824);
        assert_eq!(parse_size("invalid"), 0);
    }
}
