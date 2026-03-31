//! Grafo de dependências

use std::collections::{HashMap, HashSet};

use crate::models::Package;

/// Grafo de dependências do sistema
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Mapa de pacote -> suas dependências
    dependencies: HashMap<String, HashSet<String>>,

    /// Mapa reverso: pacote -> pacotes que dependem dele
    reverse_dependencies: HashMap<String, HashSet<String>>,

    /// Set de pacotes instalados explicitamente
    explicit_packages: HashSet<String>,
}

impl DependencyGraph {
    /// Constrói o grafo a partir da lista de pacotes
    pub fn build(packages: &[Package]) -> Self {
        let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();
        let mut reverse_dependencies: HashMap<String, HashSet<String>> = HashMap::new();
        let mut explicit_packages: HashSet<String> = HashSet::new();

        // Inicializar todos os pacotes no grafo
        for pkg in packages {
            dependencies.entry(pkg.name.clone()).or_default();
            reverse_dependencies.entry(pkg.name.clone()).or_default();

            if pkg.is_explicit() {
                explicit_packages.insert(pkg.name.clone());
            }
        }

        // Construir relacionamentos
        for pkg in packages {
            let deps: HashSet<String> = pkg.dependencies.iter().cloned().collect();

            // Adicionar dependências diretas
            dependencies.insert(pkg.name.clone(), deps.clone());

            // Atualizar dependências reversas
            for dep in &deps {
                reverse_dependencies
                    .entry(dep.clone())
                    .or_default()
                    .insert(pkg.name.clone());
            }
        }

        Self {
            dependencies,
            reverse_dependencies,
            explicit_packages,
        }
    }

    /// Retorna todos os pacotes órfãos
    ///
    /// Um pacote é órfão se:
    /// 1. Foi instalado como dependência (não explícito)
    /// 2. Nenhum outro pacote depende dele
    pub fn find_orphans(&self) -> Vec<String> {
        let mut orphans = Vec::new();

        for (pkg_name, reverse_deps) in &self.reverse_dependencies {
            // Se não é explícito e ninguém depende dele
            if !self.explicit_packages.contains(pkg_name) && reverse_deps.is_empty() {
                orphans.push(pkg_name.clone());
            }
        }

        orphans.sort();
        orphans
    }

    /// Calcula o impacto da remoção de um pacote
    pub fn removal_impact(&self, package: &str) -> RemovalImpact {
        let affected = self.get_reverse_dependencies(package);
        let is_safe = affected.is_empty();

        let mut warnings = Vec::new();
        if !is_safe {
            warnings.push(format!(
                "Pacote '{}' é dependência de: {}",
                package,
                affected.iter().cloned().collect::<Vec<_>>().join(", ")
            ));
        }

        RemovalImpact {
            affected_packages: affected.into_iter().collect(),
            is_safe,
            warnings,
        }
    }

    /// Detecta ciclos de dependências usando DFS
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for pkg in self.dependencies.keys() {
            if !visited.contains(pkg) {
                self.dfs_cycle(pkg, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    /// DFS auxiliar para detecção de ciclos
    fn dfs_cycle(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.dfs_cycle(dep, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(dep) {
                    // Ciclo encontrado
                    let cycle_start = path.iter().position(|x| x == dep).unwrap();
                    let cycle: Vec<String> = path[cycle_start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// Retorna todas as dependências transitivas de um pacote
    pub fn transitive_dependencies(&self, package: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut stack = vec![package.to_string()];

        while let Some(current) = stack.pop() {
            if let Some(deps) = self.dependencies.get(&current) {
                for dep in deps {
                    if result.insert(dep.clone()) {
                        stack.push(dep.clone());
                    }
                }
            }
        }

        result
    }

    /// Retorna as dependências diretas de um pacote
    pub fn get_dependencies(&self, package: &str) -> HashSet<String> {
        self.dependencies.get(package).cloned().unwrap_or_default()
    }

    /// Retorna os pacotes que dependem do pacote especificado
    pub fn get_reverse_dependencies(&self, package: &str) -> HashSet<String> {
        self.reverse_dependencies
            .get(package)
            .cloned()
            .unwrap_or_default()
    }

    /// Verifica se um pacote existe no grafo
    pub fn contains(&self, package: &str) -> bool {
        self.dependencies.contains_key(package)
    }

    /// Retorna o número total de pacotes no grafo
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Verifica se o grafo está vazio
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }
}

/// Impacto da remoção de um pacote
#[derive(Debug, Clone)]
pub struct RemovalImpact {
    /// Pacotes que serão afetados pela remoção
    pub affected_packages: Vec<String>,

    /// Se a remoção é segura (não afeta outros pacotes)
    pub is_safe: bool,

    /// Avisos sobre a remoção
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::InstallReason;

    fn create_test_packages() -> Vec<Package> {
        vec![
            {
                let mut pkg = Package::new("base".to_string(), "1.0".to_string());
                pkg.install_reason = InstallReason::Explicit;
                pkg
            },
            {
                let mut pkg = Package::new("vim".to_string(), "9.0".to_string());
                pkg.install_reason = InstallReason::Explicit;
                pkg.dependencies = vec!["ncurses".to_string()];
                pkg
            },
            {
                let mut pkg = Package::new("ncurses".to_string(), "6.4".to_string());
                pkg.install_reason = InstallReason::Dependency;
                pkg
            },
            {
                let mut pkg = Package::new("orphan-lib".to_string(), "1.0".to_string());
                pkg.install_reason = InstallReason::Dependency;
                pkg
            },
        ]
    }

    #[test]
    fn test_build_graph() {
        let packages = create_test_packages();
        let graph = DependencyGraph::build(&packages);

        assert_eq!(graph.len(), 4);
        assert!(graph.contains("vim"));
        assert!(graph.contains("ncurses"));
    }

    #[test]
    fn test_find_orphans() {
        let packages = create_test_packages();
        let graph = DependencyGraph::build(&packages);

        let orphans = graph.find_orphans();

        // orphan-lib é órfão (dependência não usada)
        assert!(orphans.contains(&"orphan-lib".to_string()));

        // ncurses NÃO é órfão (vim depende dele)
        assert!(!orphans.contains(&"ncurses".to_string()));

        // vim e base não são órfãos (são explícitos)
        assert!(!orphans.contains(&"vim".to_string()));
        assert!(!orphans.contains(&"base".to_string()));
    }

    #[test]
    fn test_transitive_dependencies() {
        let mut packages = create_test_packages();
        // ncurses depende de glibc
        packages[2].dependencies = vec!["glibc".to_string()];
        // Adicionar glibc
        let mut glibc = Package::new("glibc".to_string(), "2.38".to_string());
        glibc.install_reason = InstallReason::Dependency;
        packages.push(glibc);

        let graph = DependencyGraph::build(&packages);

        let vim_deps = graph.transitive_dependencies("vim");
        assert!(vim_deps.contains("ncurses"));
        assert!(vim_deps.contains("glibc"));
    }

    #[test]
    fn test_removal_impact() {
        let packages = create_test_packages();
        let graph = DependencyGraph::build(&packages);

        // Remover ncurses afeta vim
        let impact = graph.removal_impact("ncurses");
        assert!(!impact.is_safe);
        assert!(impact.affected_packages.contains(&"vim".to_string()));

        // Remover orphan-lib é seguro
        let impact = graph.removal_impact("orphan-lib");
        assert!(impact.is_safe);
        assert!(impact.affected_packages.is_empty());
    }
}
