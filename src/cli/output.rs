//! Formatação de output para terminal

use crate::models::{OrphanPackage, RiskLevel, SystemReport};
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};

/// Opções de output
#[derive(Default)]
pub struct OutputOptions {
    pub verbose: bool,
    pub quiet: bool,
    pub json: bool,
    pub sort_by_size: bool,
}

/// Imprime o relatório do sistema
pub fn print_report(report: &SystemReport, options: &OutputOptions) {
    if options.json {
        if let Ok(json) = report.to_json() {
            println!("{}", json);
        }
        return;
    }

    if options.quiet {
        print_orphan_names(&report.orphans);
        return;
    }

    print_header();
    print_stats(report);
    print_orphans_table(&report.orphans, options.sort_by_size);
}

/// Imprime cabeçalho
fn print_header() {
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║              OxidClean - Relatório do Sistema             ║".cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
}

/// Imprime estatísticas resumidas
pub fn print_stats(report: &SystemReport) {
    println!("{}", "📊 Estatísticas".bold());
    println!(
        "   Total de pacotes instalados: {}",
        report.total_packages.to_string().cyan()
    );
    println!(
        "   Pacotes órfãos encontrados:  {}",
        if report.orphans.is_empty() {
            "0".green().to_string()
        } else {
            report.orphans.len().to_string().yellow().to_string()
        }
    );
    println!(
        "   Espaço recuperável:          {}",
        crate::utils::humanize_bytes(report.recoverable_space).green()
    );
    println!();
}

/// Imprime tabela de órfãos
pub fn print_orphans_table(orphans: &[OrphanPackage], sort_by_size: bool) {
    if orphans.is_empty() {
        println!("{}", "✓ Nenhum pacote órfão encontrado!".green());
        return;
    }

    let mut orphans = orphans.to_vec();
    if sort_by_size {
        orphans.sort_by(|a, b| b.size.cmp(&a.size));
    }

    println!("{}", "📦 Pacotes Órfãos".bold());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Nome", "Versão", "Tamanho", "Risco"]);

    for orphan in &orphans {
        let risk_str = match orphan.risk_level {
            RiskLevel::Safe => "Seguro".green().to_string(),
            RiskLevel::Caution => "Atenção".yellow().to_string(),
            RiskLevel::Critical => "Crítico".red().to_string(),
        };

        table.add_row(vec![
            orphan.name.clone(),
            orphan.version.clone(),
            crate::utils::humanize_bytes(orphan.size),
            risk_str,
        ]);
    }

    println!("{table}");
}

/// Imprime apenas nomes dos órfãos (modo quiet)
fn print_orphan_names(orphans: &[OrphanPackage]) {
    for orphan in orphans {
        println!("{}", orphan.name);
    }
}

/// Imprime mensagem de sucesso
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Imprime mensagem de aviso
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

/// Imprime mensagem de erro
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Imprime mensagem de informação
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}
