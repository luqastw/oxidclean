//! Gerenciador de cache do Pacman

use crate::dal::pacman_config::PacmanConfig;
use crate::models::CacheStats;
use crate::Result;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Entrada de cache
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Nome do pacote
    pub name: String,

    /// Versão
    pub version: String,

    /// Caminho do arquivo
    pub path: PathBuf,

    /// Tamanho em bytes
    pub size: u64,
}

/// Gerenciador de cache do Pacman
pub struct CacheManager {
    cache_dir: PathBuf,
    keep_versions: usize,
}

impl CacheManager {
    /// Cria um novo gerenciador de cache
    pub fn new() -> Result<Self> {
        let config = PacmanConfig::load().unwrap_or_default();
        Ok(Self {
            cache_dir: config.cache_dir,
            keep_versions: 3,
        })
    }

    /// Cria um gerenciador com configurações customizadas
    pub fn with_options(cache_dir: PathBuf, keep_versions: usize) -> Self {
        Self {
            cache_dir,
            keep_versions,
        }
    }

    /// Escaneia o cache e retorna estatísticas
    pub fn scan(&self) -> Result<CacheStats> {
        let mut stats = CacheStats::new();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        let entries = self.list_cache_entries()?;

        stats.total_packages = entries.len();
        stats.total_size = entries.iter().map(|e| e.size).sum();

        // TODO: Comparar com pacotes instalados para determinar unused
        // Por enquanto, retorna estatísticas básicas

        Ok(stats)
    }

    /// Lista todas as entradas do cache
    pub fn list_cache_entries(&self) -> Result<Vec<CacheEntry>> {
        let mut entries = Vec::new();

        if !self.cache_dir.exists() {
            return Ok(entries);
        }

        for entry in WalkDir::new(&self.cache_dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Apenas arquivos .pkg.tar.*
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains(".pkg.tar.") {
                    if let Some(cache_entry) = self.parse_cache_filename(path) {
                        entries.push(cache_entry);
                    }
                }
            }
        }

        Ok(entries)
    }

    /// Encontra pacotes em cache que não estão instalados
    pub fn find_unused(&self, installed_packages: &HashSet<String>) -> Result<Vec<CacheEntry>> {
        let entries = self.list_cache_entries()?;

        let unused: Vec<CacheEntry> = entries
            .into_iter()
            .filter(|e| !installed_packages.contains(&e.name))
            .collect();

        Ok(unused)
    }

    /// Encontra versões antigas de pacotes em cache
    pub fn find_old_versions(&self) -> Result<Vec<CacheEntry>> {
        let entries = self.list_cache_entries()?;

        // Agrupar por nome de pacote
        let mut by_name: std::collections::HashMap<String, Vec<CacheEntry>> =
            std::collections::HashMap::new();

        for entry in entries {
            by_name.entry(entry.name.clone()).or_default().push(entry);
        }

        let mut old_versions = Vec::new();

        for (_name, mut versions) in by_name {
            // Ordenar por versão (mais recente primeiro)
            versions.sort_by(|a, b| b.version.cmp(&a.version));

            // Manter apenas keep_versions mais recentes
            if versions.len() > self.keep_versions {
                old_versions.extend(versions.into_iter().skip(self.keep_versions));
            }
        }

        Ok(old_versions)
    }

    /// Parse de nome de arquivo de cache
    fn parse_cache_filename(&self, path: &Path) -> Option<CacheEntry> {
        let filename = path.file_name()?.to_str()?;

        // Formato: nome-versão-release-arch.pkg.tar.zst
        // Exemplo: vim-9.0.1234-1-x86_64.pkg.tar.zst

        // Encontrar onde começa a extensão .pkg.tar
        let pkg_idx = filename.find(".pkg.tar")?;
        let base = &filename[..pkg_idx];

        // Dividir em partes pelo hífen
        let parts: Vec<&str> = base.rsplitn(4, '-').collect();
        if parts.len() < 3 {
            return None;
        }

        // parts[0] = arch, parts[1] = release, parts[2] = version, parts[3..] = name
        let _arch = parts[0];
        let release = parts[1];
        let version = parts[2];
        let name = if parts.len() > 3 {
            parts[3..]
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<_>>()
                .join("-")
        } else {
            return None;
        };

        let size = path.metadata().ok()?.len();

        Some(CacheEntry {
            name,
            version: format!("{}-{}", version, release),
            path: path.to_path_buf(),
            size,
        })
    }

    /// Remove entradas de cache
    pub fn clean(&self, entries: &[CacheEntry]) -> Result<u64> {
        let mut freed = 0u64;

        for entry in entries {
            if entry.path.exists() {
                freed += entry.size;
                fs::remove_file(&entry.path)?;
            }
        }

        Ok(freed)
    }

    /// Retorna o diretório de cache
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new().expect("Falha ao criar CacheManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_cache() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CacheManager::with_options(temp_dir.path().to_path_buf(), 3);

        let stats = manager.scan().unwrap();
        assert_eq!(stats.total_packages, 0);
        assert_eq!(stats.total_size, 0);
    }

    #[test]
    fn test_parse_cache_filename() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CacheManager::with_options(temp_dir.path().to_path_buf(), 3);

        // Criar arquivo de teste
        let pkg_path = temp_dir.path().join("vim-9.0.1234-1-x86_64.pkg.tar.zst");
        fs::write(&pkg_path, "test").unwrap();

        let entry = manager.parse_cache_filename(&pkg_path).unwrap();
        assert_eq!(entry.name, "vim");
        assert_eq!(entry.version, "9.0.1234-1");
    }
}
