//! Validador de segurança

use crate::dal::ConfigReader;
use crate::models::{Config, DependencyGraph, Package, RiskLevel};

/// Validador de operações
pub struct Validator {
    config: Config,
}

impl Validator {
    /// Cria um novo validador
    pub fn new() -> Self {
        Self {
            config: ConfigReader::load(),
        }
    }

    /// Cria um validador com configuração customizada
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    /// Verifica se é seguro remover um pacote
    pub fn is_safe_to_remove(&self, package: &Package, graph: &DependencyGraph) -> bool {
        // Não é seguro se é protegido
        if self.config.is_protected(&package.name) {
            return false;
        }

        // Não é seguro se outros pacotes dependem dele
        let reverse_deps = graph.get_reverse_dependencies(&package.name);
        if !reverse_deps.is_empty() {
            return false;
        }

        true
    }

    /// Verifica se um pacote está protegido
    pub fn is_protected(&self, package: &Package) -> bool {
        self.config.is_protected(&package.name)
    }

    /// Classifica o nível de risco de remoção
    pub fn classify_risk(&self, package: &Package, graph: &DependencyGraph) -> RiskLevel {
        // Crítico se protegido
        if self.config.is_protected(&package.name) {
            return RiskLevel::Critical;
        }

        // Crítico se tem dependentes
        let reverse_deps = graph.get_reverse_dependencies(&package.name);
        if !reverse_deps.is_empty() {
            return RiskLevel::Critical;
        }

        // Atenção para pacotes de sistema
        for pattern in super::CAUTION_PATTERNS {
            if package.name.contains(pattern) {
                return RiskLevel::Caution;
            }
        }

        // Atenção para libs compartilhadas comuns
        if package.name.starts_with("lib") || package.name.contains("-lib") {
            return RiskLevel::Caution;
        }

        RiskLevel::Safe
    }

    /// Valida uma lista de pacotes para remoção
    pub fn validate_removal_batch(
        &self,
        packages: &[Package],
        graph: &DependencyGraph,
    ) -> ValidationResult {
        let mut result = ValidationResult::default();

        for pkg in packages {
            if self.config.is_protected(&pkg.name) {
                result.protected.push(pkg.name.clone());
            } else if !self.is_safe_to_remove(pkg, graph) {
                result.unsafe_packages.push(pkg.name.clone());
            } else {
                result.safe.push(pkg.name.clone());
            }
        }

        result
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// Resultado de validação de lote
#[derive(Debug, Default)]
pub struct ValidationResult {
    /// Pacotes seguros para remover
    pub safe: Vec<String>,

    /// Pacotes protegidos (não devem ser removidos)
    pub protected: Vec<String>,

    /// Pacotes inseguros (têm dependentes)
    pub unsafe_packages: Vec<String>,
}

impl ValidationResult {
    /// Verifica se todos os pacotes são seguros
    pub fn all_safe(&self) -> bool {
        self.protected.is_empty() && self.unsafe_packages.is_empty()
    }

    /// Retorna o total de pacotes verificados
    pub fn total(&self) -> usize {
        self.safe.len() + self.protected.len() + self.unsafe_packages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::InstallReason;

    fn create_test_packages() -> Vec<Package> {
        vec![
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
                let mut pkg = Package::new("orphan".to_string(), "1.0".to_string());
                pkg.install_reason = InstallReason::Dependency;
                pkg
            },
        ]
    }

    #[test]
    fn test_is_protected() {
        let validator = Validator::new();
        let base = Package::new("base".to_string(), "1.0".to_string());
        let vim = Package::new("vim".to_string(), "9.0".to_string());

        assert!(validator.is_protected(&base));
        assert!(!validator.is_protected(&vim));
    }

    #[test]
    fn test_is_safe_to_remove() {
        let packages = create_test_packages();
        let graph = DependencyGraph::build(&packages);
        let validator = Validator::new();

        // ncurses não é seguro (vim depende dele)
        let ncurses = &packages[1];
        assert!(!validator.is_safe_to_remove(ncurses, &graph));

        // orphan é seguro (ninguém depende dele)
        let orphan = &packages[2];
        assert!(validator.is_safe_to_remove(orphan, &graph));
    }

    #[test]
    fn test_classify_risk() {
        let packages = create_test_packages();
        let graph = DependencyGraph::build(&packages);
        let validator = Validator::new();

        // orphan é seguro
        let orphan = &packages[2];
        assert_eq!(validator.classify_risk(orphan, &graph), RiskLevel::Safe);

        // ncurses é crítico (tem dependente)
        let ncurses = &packages[1];
        assert_eq!(
            validator.classify_risk(ncurses, &graph),
            RiskLevel::Critical
        );
    }

    #[test]
    fn test_validate_batch() {
        let packages = create_test_packages();
        let graph = DependencyGraph::build(&packages);
        let validator = Validator::new();

        let result = validator.validate_removal_batch(&packages, &graph);

        // orphan é seguro
        assert!(result.safe.contains(&"orphan".to_string()));

        // ncurses não é seguro (vim depende)
        assert!(result.unsafe_packages.contains(&"ncurses".to_string()));
    }
}
