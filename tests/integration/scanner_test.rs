//! Testes de integração para o Scanner

use oxidclean::dal::PacmanReader;
use oxidclean::models::DependencyGraph;
use std::fs;
use tempfile::TempDir;

fn create_mock_db() -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().to_path_buf();

    // Criar pacote explícito
    create_mock_package(&db_path, "vim", "9.0", "0", vec!["ncurses"]);

    // Criar pacote dependência (usado)
    create_mock_package(&db_path, "ncurses", "6.4", "1", vec![]);

    // Criar pacote órfão
    create_mock_package(&db_path, "orphan-lib", "1.0", "1", vec![]);

    // Criar pacote base explícito
    create_mock_package(&db_path, "base", "1.0", "0", vec![]);

    (temp_dir, db_path)
}

fn create_mock_package(
    db_path: &std::path::Path,
    name: &str,
    version: &str,
    reason: &str,
    deps: Vec<&str>,
) {
    let pkg_dir = db_path.join(format!("{}-{}", name, version));
    fs::create_dir_all(&pkg_dir).unwrap();

    let deps_section = if deps.is_empty() {
        String::new()
    } else {
        format!("\n%DEPENDS%\n{}\n", deps.join("\n"))
    };

    let desc_content = format!(
        r#"%NAME%
{}

%VERSION%
{}

%DESC%
Test package {}

%ISIZE%
1024

%REASON%
{}
{}
"#,
        name, version, name, reason, deps_section
    );

    fs::write(pkg_dir.join("desc"), desc_content).unwrap();
}

#[test]
fn test_pacman_reader_with_mock_db() {
    let (_temp_dir, db_path) = create_mock_db();

    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();

    assert_eq!(packages.len(), 4);

    // Verificar pacote vim
    let vim = packages.iter().find(|p| p.name == "vim").unwrap();
    assert_eq!(vim.version, "9.0");
    assert!(vim.is_explicit());
    assert!(vim.dependencies.contains(&"ncurses".to_string()));

    // Verificar pacote orphan-lib
    let orphan = packages.iter().find(|p| p.name == "orphan-lib").unwrap();
    assert!(orphan.is_dependency());
}

#[test]
fn test_dependency_graph_orphan_detection() {
    let (_temp_dir, db_path) = create_mock_db();

    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();
    let graph = DependencyGraph::build(&packages);

    let orphans = graph.find_orphans();

    // orphan-lib deve ser detectado como órfão
    assert!(orphans.contains(&"orphan-lib".to_string()));

    // ncurses NÃO deve ser órfão (vim depende dele)
    assert!(!orphans.contains(&"ncurses".to_string()));

    // vim e base não são órfãos (são explícitos)
    assert!(!orphans.contains(&"vim".to_string()));
    assert!(!orphans.contains(&"base".to_string()));
}

#[test]
fn test_dependency_graph_reverse_deps() {
    let (_temp_dir, db_path) = create_mock_db();

    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();
    let graph = DependencyGraph::build(&packages);

    // vim depende de ncurses
    let ncurses_reverse = graph.get_reverse_dependencies("ncurses");
    assert!(ncurses_reverse.contains("vim"));

    // orphan-lib não tem reverse deps
    let orphan_reverse = graph.get_reverse_dependencies("orphan-lib");
    assert!(orphan_reverse.is_empty());
}

#[test]
fn test_removal_impact_safe() {
    let (_temp_dir, db_path) = create_mock_db();

    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();
    let graph = DependencyGraph::build(&packages);

    // Remover orphan-lib é seguro
    let impact = graph.removal_impact("orphan-lib");
    assert!(impact.is_safe);
    assert!(impact.affected_packages.is_empty());
}

#[test]
fn test_removal_impact_unsafe() {
    let (_temp_dir, db_path) = create_mock_db();

    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();
    let graph = DependencyGraph::build(&packages);

    // Remover ncurses afeta vim
    let impact = graph.removal_impact("ncurses");
    assert!(!impact.is_safe);
    assert!(impact.affected_packages.contains(&"vim".to_string()));
}

#[test]
fn test_transitive_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().to_path_buf();

    // Criar cadeia: vim -> ncurses -> glibc
    create_mock_package(&db_path, "vim", "9.0", "0", vec!["ncurses"]);
    create_mock_package(&db_path, "ncurses", "6.4", "1", vec!["glibc"]);
    create_mock_package(&db_path, "glibc", "2.38", "1", vec![]);

    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();
    let graph = DependencyGraph::build(&packages);

    let vim_deps = graph.transitive_dependencies("vim");
    assert!(vim_deps.contains("ncurses"));
    assert!(vim_deps.contains("glibc"));
}
