//! Analisador de dependências

use crate::dal::PacmanReader;
use crate::models::{DependencyGraph, Package};
use crate::Result;
use std::collections::HashSet;

/// Análise detalhada de um pacote
#[derive(Debug, Clone)]
pub struct PackageAnalysis {
    /// Nome do pacote
    pub package_name: String,

    /// Versão do pacote
    pub version: String,

    /// Tamanho do pacote
    pub size: u64,

    /// Dependências diretas
    pub direct_dependencies: Vec<String>,

    /// Dependências transitivas (indiretas)
    pub transitive_dependencies: Vec<String>,

    /// Tamanho total incluindo dependências
    pub total_size: u64,

    /// Pacotes que dependem deste pacote
    pub reverse_dependencies: Vec<String>,

    /// Se o pacote é explícito
    pub is_explicit: bool,

    /// Descrição do pacote
    pub description: Option<String>,
}

/// Analisador de dependências
pub struct Analyzer {
    packages: Vec<Package>,
    graph: DependencyGraph,
}

impl Analyzer {
    /// Cria um novo analisador
    pub fn new() -> Result<Self> {
        let reader = PacmanReader::new()?;
        let packages = reader.read_all_packages()?;
        let graph = DependencyGraph::build(&packages);

        Ok(Self { packages, graph })
    }

    /// Cria um analisador com pacotes fornecidos (útil para testes)
    pub fn with_packages(packages: Vec<Package>) -> Self {
        let graph = DependencyGraph::build(&packages);
        Self { packages, graph }
    }

    /// Analisa um pacote específico
    pub fn analyze_package(&self, package_name: &str) -> Result<PackageAnalysis> {
        // Encontrar o pacote
        let pkg = self
            .packages
            .iter()
            .find(|p| p.name == package_name)
            .ok_or_else(|| crate::OxidCleanError::PackageNotFound(package_name.to_string()))?;

        // Dependências diretas
        let direct_deps = self.graph.get_dependencies(package_name);

        // Dependências transitivas
        let transitive_deps = self.graph.transitive_dependencies(package_name);
        let transitive_only: Vec<String> =
            transitive_deps.difference(&direct_deps).cloned().collect();

        // Dependências reversas
        let reverse_deps = self.graph.get_reverse_dependencies(package_name);

        // Calcular tamanho total
        let total_size = self.calculate_total_size(package_name);

        Ok(PackageAnalysis {
            package_name: pkg.name.clone(),
            version: pkg.version.clone(),
            size: pkg.size,
            direct_dependencies: direct_deps.into_iter().collect(),
            transitive_dependencies: transitive_only,
            total_size,
            reverse_dependencies: reverse_deps.into_iter().collect(),
            is_explicit: pkg.is_explicit(),
            description: pkg.description.clone(),
        })
    }

    /// Calcula o tamanho total de um pacote incluindo dependências
    fn calculate_total_size(&self, package_name: &str) -> u64 {
        let mut total: u64 = 0;
        let mut visited = HashSet::new();
        visited.insert(package_name.to_string());

        // Tamanho do pacote
        if let Some(pkg) = self.packages.iter().find(|p| p.name == package_name) {
            total += pkg.size;
        }

        // Tamanho das dependências
        let deps = self.graph.transitive_dependencies(package_name);
        for dep_name in deps {
            if visited.insert(dep_name.clone()) {
                if let Some(dep_pkg) = self.packages.iter().find(|p| p.name == dep_name) {
                    total += dep_pkg.size;
                }
            }
        }

        total
    }

    /// Encontra dependências redundantes no sistema
    pub fn find_redundant_dependencies(&self) -> Vec<String> {
        // Dependência redundante: instalada como dependência de múltiplos pacotes
        // mas todos esses pacotes também têm como dependência transitiva
        // (implementação simplificada)
        Vec::new() // TODO: Implementar lógica completa
    }

    /// Detecta ciclos de dependências
    pub fn detect_circular_dependencies(&self) -> Vec<Vec<String>> {
        self.graph.detect_cycles()
    }

    /// Retorna o grafo de dependências
    pub fn graph(&self) -> &DependencyGraph {
        &self.graph
    }

    /// Retorna todos os pacotes
    pub fn packages(&self) -> &[Package] {
        &self.packages
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::InstallReason;

    fn create_test_packages() -> Vec<Package> {
        vec![
            {
                let mut pkg = Package::new("app".to_string(), "1.0".to_string());
                pkg.install_reason = InstallReason::Explicit;
                pkg.dependencies = vec!["lib-a".to_string()];
                pkg.size = 1000;
                pkg
            },
            {
                let mut pkg = Package::new("lib-a".to_string(), "1.0".to_string());
                pkg.install_reason = InstallReason::Dependency;
                pkg.dependencies = vec!["lib-b".to_string()];
                pkg.size = 500;
                pkg
            },
            {
                let mut pkg = Package::new("lib-b".to_string(), "1.0".to_string());
                pkg.install_reason = InstallReason::Dependency;
                pkg.size = 200;
                pkg
            },
        ]
    }

    #[test]
    fn test_analyze_package() {
        let packages = create_test_packages();
        let analyzer = Analyzer::with_packages(packages);

        let analysis = analyzer.analyze_package("app").unwrap();

        assert_eq!(analysis.package_name, "app");
        assert!(analysis.direct_dependencies.contains(&"lib-a".to_string()));
        assert!(analysis
            .transitive_dependencies
            .contains(&"lib-b".to_string()));
        assert!(analysis.is_explicit);
    }

    #[test]
    fn test_total_size() {
        let packages = create_test_packages();
        let analyzer = Analyzer::with_packages(packages);

        let analysis = analyzer.analyze_package("app").unwrap();

        // app (1000) + lib-a (500) + lib-b (200) = 1700
        assert_eq!(analysis.total_size, 1700);
    }

    #[test]
    fn test_package_not_found() {
        let packages = create_test_packages();
        let analyzer = Analyzer::with_packages(packages);

        let result = analyzer.analyze_package("nonexistent");
        assert!(result.is_err());
    }
}
