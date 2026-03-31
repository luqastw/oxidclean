//! Benchmarks para OxidClean

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use oxidclean::dal::PacmanReader;
use oxidclean::models::DependencyGraph;
use std::fs;
use tempfile::TempDir;

/// Cria um banco de dados mock com N pacotes
fn create_mock_db(num_packages: usize) -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().to_path_buf();

    for i in 0..num_packages {
        let name = format!("package-{}", i);
        let version = "1.0";
        let reason = if i % 5 == 0 { "0" } else { "1" }; // 20% explícitos

        // Criar dependências (cada pacote depende de alguns anteriores)
        let deps: Vec<String> = if i > 0 {
            (0..std::cmp::min(3, i))
                .map(|j| format!("package-{}", i - j - 1))
                .collect()
        } else {
            vec![]
        };

        create_mock_package(&db_path, &name, version, reason, &deps);
    }

    (temp_dir, db_path)
}

fn create_mock_package(
    db_path: &std::path::Path,
    name: &str,
    version: &str,
    reason: &str,
    deps: &[String],
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
102400

%REASON%
{}
{}
"#,
        name, version, name, reason, deps_section
    );

    fs::write(pkg_dir.join("desc"), desc_content).unwrap();
}

/// Benchmark de leitura de pacotes
fn bench_read_packages(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_packages");
    group.sample_size(10);

    for size in [50, 100, 200].iter() {
        let (_temp_dir, db_path) = create_mock_db(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let reader = PacmanReader::with_path(&db_path).unwrap();
                black_box(reader.read_all_packages().unwrap())
            })
        });
    }

    group.finish();
}

/// Benchmark de construção do grafo de dependências
fn bench_build_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_graph");
    group.sample_size(10);

    for size in [50, 100, 200].iter() {
        let (_temp_dir, db_path) = create_mock_db(*size);
        let reader = PacmanReader::with_path(&db_path).unwrap();
        let packages = reader.read_all_packages().unwrap();

        group.bench_with_input(BenchmarkId::from_parameter(size), &packages, |b, pkgs| {
            b.iter(|| black_box(DependencyGraph::build(pkgs)))
        });
    }

    group.finish();
}

/// Benchmark de detecção de órfãos
fn bench_find_orphans(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_orphans");
    group.sample_size(10);

    for size in [50, 100, 200].iter() {
        let (_temp_dir, db_path) = create_mock_db(*size);
        let reader = PacmanReader::with_path(&db_path).unwrap();
        let packages = reader.read_all_packages().unwrap();
        let graph = DependencyGraph::build(&packages);

        group.bench_with_input(BenchmarkId::from_parameter(size), &graph, |b, g| {
            b.iter(|| black_box(g.find_orphans()))
        });
    }

    group.finish();
}

/// Benchmark de dependências transitivas
fn bench_transitive_dependencies(c: &mut Criterion) {
    let mut group = c.benchmark_group("transitive_deps");
    group.sample_size(10);

    let (_temp_dir, db_path) = create_mock_db(100);
    let reader = PacmanReader::with_path(&db_path).unwrap();
    let packages = reader.read_all_packages().unwrap();
    let graph = DependencyGraph::build(&packages);

    group.bench_function("100_packages", |b| {
        b.iter(|| black_box(graph.transitive_dependencies("package-80")))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_read_packages,
    bench_build_graph,
    bench_find_orphans,
    bench_transitive_dependencies
);
criterion_main!(benches);
