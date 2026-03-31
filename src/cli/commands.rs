//! Definição de comandos CLI usando clap

use clap::{Parser, Subcommand, ValueEnum};

/// OxidClean - Gerenciador de pacotes órfãos do Arch Linux
#[derive(Parser)]
#[command(name = "oxidclean")]
#[command(author = "OxidClean Contributors")]
#[command(version)]
#[command(about = "Gerenciador de pacotes órfãos do Arch Linux", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Modo verboso - exibe mais detalhes
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Modo silencioso - exibe apenas resultados
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

/// Shells suportados para completion
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell
    Powershell,
    /// Elvish shell
    Elvish,
}

impl From<Shell> for clap_complete::Shell {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::Powershell => clap_complete::Shell::PowerShell,
            Shell::Elvish => clap_complete::Shell::Elvish,
        }
    }
}

/// Subcomandos disponíveis
#[derive(Subcommand)]
pub enum Commands {
    /// Escanear sistema e exibir relatório de pacotes órfãos
    Scan {
        /// Ordenar resultados por tamanho em disco
        #[arg(long)]
        sort_by_size: bool,

        /// Saída em formato JSON
        #[arg(long)]
        json: bool,

        /// Exportar relatório para arquivo
        #[arg(long, value_name = "FILE")]
        export: Option<String>,
    },

    /// Remover pacotes órfãos de forma interativa
    Clean {
        /// Simular ações sem executá-las
        #[arg(long)]
        dry_run: bool,

        /// Confirmar automaticamente todas as ações
        #[arg(short, long)]
        yes: bool,
    },

    /// Analisar dependências de um pacote específico
    Analyze {
        /// Nome do pacote a ser analisado
        package: String,

        /// Saída em formato JSON
        #[arg(long)]
        json: bool,
    },

    /// Gerenciar cache do pacman
    Cache {
        /// Mostrar apenas estatísticas do cache
        #[arg(long)]
        stats: bool,

        /// Limpar cache de pacotes antigos
        #[arg(long)]
        clean: bool,

        /// Simular ações sem executá-las
        #[arg(long)]
        dry_run: bool,

        /// Confirmar automaticamente todas as ações
        #[arg(short, long)]
        yes: bool,

        /// Número de versões a manter por pacote (padrão: 3)
        #[arg(long, default_value = "3")]
        keep: usize,
    },

    /// Listar todos os pacotes órfãos detectados
    List {
        /// Ordenar por tamanho em disco
        #[arg(long)]
        sort_by_size: bool,
    },

    /// Gerar scripts de auto-completar para shell
    Completion {
        /// Shell para gerar completion
        #[arg(value_enum)]
        shell: Shell,
    },
}
