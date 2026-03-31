//! OxidClean - Gerenciador de pacotes órfãos do Arch Linux
//!
//! Uma ferramenta de terminal feita em Rust para gerenciar pacotes do Arch Linux,
//! servindo como otimizador de dependências e removedor de pacotes que não são
//! utilizados e apenas estão ocupando espaço.

use clap::Parser;
use colored::Colorize;
use oxidclean::cli::output::{self, OutputOptions};
use oxidclean::cli::{Cli, Commands};
use oxidclean::core::{Analyzer, Cleaner, Scanner};
use oxidclean::utils::system_info;

fn main() {
    // Inicializar logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    // Parse argumentos
    let cli = Cli::parse();

    // Executar comando
    if let Err(e) = run(cli) {
        output::print_error(&format!("{}", e));
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> oxidclean::Result<()> {
    // Verificar sistema suportado (exceto para help/version)
    if !matches!(cli.command, Commands::List { .. }) {
        // Verificação de sistema pode ser relaxada para list
    }

    let output_opts = OutputOptions {
        verbose: cli.verbose,
        quiet: cli.quiet,
        json: false,
        sort_by_size: false,
    };

    match cli.command {
        Commands::Scan {
            sort_by_size,
            json,
            export,
        } => cmd_scan(
            OutputOptions {
                sort_by_size,
                json,
                ..output_opts
            },
            export,
        ),

        Commands::Clean { dry_run, yes } => cmd_clean(dry_run, yes, &output_opts),

        Commands::Analyze { package, json } => cmd_analyze(
            &package,
            OutputOptions {
                json,
                ..output_opts
            },
        ),

        Commands::Cache { stats, clean } => cmd_cache(stats, clean, &output_opts),

        Commands::List { sort_by_size } => cmd_list(OutputOptions {
            sort_by_size,
            ..output_opts
        }),
    }
}

/// Comando: scan
fn cmd_scan(opts: OutputOptions, export: Option<String>) -> oxidclean::Result<()> {
    // Verificar sistema
    let distro = system_info::check_system_support()?;
    if !opts.quiet && !opts.json {
        output::print_info(&format!("Sistema detectado: {}", distro));
    }

    let mut scanner = Scanner::new()?;
    let report = scanner.scan()?;

    // Exportar se solicitado
    if let Some(path) = export {
        let content = if path.ends_with(".json") {
            report.to_json()?
        } else {
            report.to_markdown()
        };

        std::fs::write(&path, content)?;
        output::print_success(&format!("Relatório exportado para: {}", path));
    }

    // Exibir relatório
    output::print_report(&report, &opts);

    Ok(())
}

/// Comando: clean
fn cmd_clean(dry_run: bool, yes: bool, opts: &OutputOptions) -> oxidclean::Result<()> {
    // Verificar sistema
    let distro = system_info::check_system_support()?;
    if !opts.quiet {
        output::print_info(&format!("Sistema detectado: {}", distro));
    }

    // Obter órfãos
    let mut scanner = Scanner::new()?;
    let report = scanner.scan()?;

    if report.orphans.is_empty() {
        output::print_success("Nenhum pacote órfão encontrado!");
        return Ok(());
    }

    if !opts.quiet {
        println!();
        println!(
            "Encontrados {} pacotes órfãos ({} recuperáveis)",
            report.orphans.len().to_string().yellow(),
            oxidclean::utils::humanize_bytes(report.recoverable_space).green()
        );
        println!();
    }

    if dry_run {
        output::print_warning("Modo DRY-RUN: nenhuma alteração será feita");
        println!();
    }

    // Obter pacotes completos
    let reader = oxidclean::dal::PacmanReader::new()?;
    let all_packages = reader.read_all_packages()?;

    let orphan_packages: Vec<_> = report
        .orphans
        .iter()
        .filter_map(|o| all_packages.iter().find(|p| p.name == o.name).cloned())
        .collect();

    // Limpar
    let mut cleaner = Cleaner::new(dry_run, yes)?;
    let result = cleaner.clean_interactive(orphan_packages)?;

    // Exibir resumo
    println!();
    println!("{}", "═".repeat(50).cyan());
    println!("{}", "Resumo da Limpeza".bold());
    println!("{}", "═".repeat(50).cyan());
    println!("  Removidos: {}", result.removed.len().to_string().green());
    println!("  Pulados:   {}", result.skipped.len().to_string().yellow());
    println!("  Falhas:    {}", result.failed.len().to_string().red());
    println!(
        "  Espaço liberado: {}",
        oxidclean::utils::humanize_bytes(result.space_freed).green()
    );

    if !result.failed.is_empty() {
        println!();
        output::print_warning("Pacotes com falha:");
        for (name, error) in &result.failed {
            println!("  - {}: {}", name.red(), error);
        }
    }

    Ok(())
}

/// Comando: analyze
fn cmd_analyze(package: &str, opts: OutputOptions) -> oxidclean::Result<()> {
    let analyzer = Analyzer::new()?;
    let analysis = analyzer.analyze_package(package)?;

    if opts.json {
        // Serializar para JSON
        let json = serde_json::json!({
            "package": analysis.package_name,
            "version": analysis.version,
            "size": analysis.size,
            "total_size": analysis.total_size,
            "is_explicit": analysis.is_explicit,
            "direct_dependencies": analysis.direct_dependencies,
            "transitive_dependencies": analysis.transitive_dependencies,
            "reverse_dependencies": analysis.reverse_dependencies,
            "description": analysis.description,
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
        return Ok(());
    }

    println!();
    println!("{}", "═".repeat(60).cyan());
    println!(
        "{}",
        format!("Análise: {} {}", analysis.package_name, analysis.version).bold()
    );
    println!("{}", "═".repeat(60).cyan());
    println!();

    if let Some(desc) = &analysis.description {
        println!("{}: {}", "Descrição".bold(), desc);
        println!();
    }

    println!(
        "{}: {}",
        "Tamanho".bold(),
        oxidclean::utils::humanize_bytes(analysis.size)
    );
    println!(
        "{}: {}",
        "Tamanho Total (com deps)".bold(),
        oxidclean::utils::humanize_bytes(analysis.total_size)
    );
    println!(
        "{}: {}",
        "Instalação".bold(),
        if analysis.is_explicit {
            "Explícita".green()
        } else {
            "Dependência".yellow()
        }
    );
    println!();

    println!("{}", "Dependências Diretas:".bold());
    if analysis.direct_dependencies.is_empty() {
        println!("  (nenhuma)");
    } else {
        for dep in &analysis.direct_dependencies {
            println!("  - {}", dep);
        }
    }
    println!();

    println!("{}", "Dependências Transitivas:".bold());
    if analysis.transitive_dependencies.is_empty() {
        println!("  (nenhuma)");
    } else {
        for dep in &analysis.transitive_dependencies {
            println!("  - {}", dep);
        }
    }
    println!();

    println!("{}", "Dependências Reversas (quem depende deste):".bold());
    if analysis.reverse_dependencies.is_empty() {
        println!("  {} (seguro para remover)", "(nenhuma)".green());
    } else {
        for dep in &analysis.reverse_dependencies {
            println!("  - {}", dep.yellow());
        }
    }

    Ok(())
}

/// Comando: cache
fn cmd_cache(stats: bool, clean: bool, opts: &OutputOptions) -> oxidclean::Result<()> {
    use oxidclean::core::cache_manager::CacheManager;

    let manager = CacheManager::new()?;

    if stats || (!stats && !clean) {
        let cache_stats = manager.scan()?;

        if opts.quiet {
            println!("{}", cache_stats.total_size);
        } else {
            println!();
            println!("{}", "Estatísticas do Cache".bold());
            println!("{}", "─".repeat(40));
            println!(
                "  Pacotes em cache: {}",
                cache_stats.total_packages.to_string().cyan()
            );
            println!(
                "  Tamanho total:    {}",
                oxidclean::utils::humanize_bytes(cache_stats.total_size).cyan()
            );
            println!(
                "  Não instalados:   {}",
                cache_stats.unused_packages.to_string().yellow()
            );
            println!(
                "  Espaço unused:    {}",
                oxidclean::utils::humanize_bytes(cache_stats.unused_size).yellow()
            );
        }
    }

    if clean {
        output::print_warning("Limpeza de cache ainda não implementada completamente");
        output::print_info("Use 'paccache -r' para limpar o cache do pacman");
    }

    Ok(())
}

/// Comando: list
fn cmd_list(opts: OutputOptions) -> oxidclean::Result<()> {
    let mut scanner = Scanner::new()?;
    let report = scanner.scan()?;

    let mut orphans = report.orphans;

    if opts.sort_by_size {
        orphans.sort_by(|a, b| b.size.cmp(&a.size));
    }

    if opts.quiet {
        for orphan in &orphans {
            println!("{}", orphan.name);
        }
    } else {
        if orphans.is_empty() {
            output::print_success("Nenhum pacote órfão encontrado!");
        } else {
            for orphan in &orphans {
                println!(
                    "{} {} ({})",
                    orphan.name,
                    orphan.version.dimmed(),
                    oxidclean::utils::humanize_bytes(orphan.size).dimmed()
                );
            }
            println!();
            println!(
                "Total: {} pacotes, {}",
                orphans.len().to_string().yellow(),
                oxidclean::utils::humanize_bytes(report.recoverable_space).green()
            );
        }
    }

    Ok(())
}
