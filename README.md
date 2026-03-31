# OxidClean

[![CI](https://github.com/seu-usuario/oxidclean/actions/workflows/ci.yml/badge.svg)](https://github.com/seu-usuario/oxidclean/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Gerenciador de pacotes órfãos para Arch Linux e derivados, escrito em Rust.

[English](#english) | [Português](#português)

---

## Português

### Sobre

OxidClean é uma ferramenta de terminal que identifica e remove pacotes instalados como dependências que não são mais necessários por nenhum outro pacote, otimizando o uso de espaço em disco no seu sistema Arch Linux.

### Funcionalidades

- **Detecção de Órfãos**: Identifica pacotes instalados como dependência que não são mais necessários
- **Classificação de Risco**: Pacotes são categorizados como Safe, Caution ou Critical
- **Análise de Dependências**: Grafo completo de dependências com detecção de ciclos
- **Gerenciamento de Cache**: Limpe versões antigas de pacotes do cache do pacman
- **Proteção de Pacotes**: Pacotes críticos do sistema são protegidos contra remoção acidental
- **Interface Visual**: Tabelas coloridas, barras de progresso e símbolos visuais

### Instalação

```bash
# Via Cargo
cargo install oxidclean

# Via AUR (quando disponível)
yay -S oxidclean
```

### Uso Rápido

```bash
# Escanear sistema e ver relatório
oxidclean scan

# Listar pacotes órfãos
oxidclean list

# Remover órfãos interativamente
sudo oxidclean clean

# Analisar um pacote específico
oxidclean analyze vim

# Ver estatísticas do cache
oxidclean cache --stats

# Limpar cache de pacotes
sudo oxidclean cache --clean
```

### Comandos

| Comando | Descrição |
|---------|-----------|
| `scan` | Escaneia o sistema e exibe relatório de pacotes órfãos |
| `clean` | Remove pacotes órfãos de forma interativa |
| `analyze <pkg>` | Analisa dependências de um pacote específico |
| `cache` | Gerencia o cache do pacman |
| `list` | Lista todos os pacotes órfãos |

### Exemplo de Saída

```
╔═══════════════════════════════════════════════════════════╗
║              OxidClean - Relatório do Sistema             ║
╚═══════════════════════════════════════════════════════════╝

📊 Estatísticas
   Total de pacotes instalados: 1396
   Pacotes órfãos encontrados:  31
   Espaço recuperável:          976.42 MB

📦 Pacotes Órfãos
┌─────────────────┬─────────┬──────────┬─────────┐
│ Nome            │ Versão  │ Tamanho  │ Risco   │
├─────────────────┼─────────┼──────────┼─────────┤
│ lib32-libpng    │ 1.6.43  │ 456.2 KB │ Seguro  │
│ python-pip      │ 24.0    │ 12.3 MB  │ Seguro  │
│ nvidia-utils    │ 550.67  │ 89.1 MB  │ Atenção │
└─────────────────┴─────────┴──────────┴─────────┘
```

### Documentação

- [Instalação](docs/pt/INSTALL.md)
- [Guia de Uso](docs/pt/USAGE.md)
- [Solução de Problemas](docs/pt/TROUBLESHOOTING.md)

---

## English

### About

OxidClean is a terminal tool that identifies and removes packages installed as dependencies that are no longer required by any other package, optimizing disk space usage on your Arch Linux system.

### Features

- **Orphan Detection**: Identifies packages installed as dependencies that are no longer needed
- **Risk Classification**: Packages are categorized as Safe, Caution, or Critical
- **Dependency Analysis**: Complete dependency graph with cycle detection
- **Cache Management**: Clean old package versions from pacman cache
- **Package Protection**: Critical system packages are protected against accidental removal
- **Visual Interface**: Colored tables, progress bars, and visual symbols

### Installation

```bash
# Via Cargo
cargo install oxidclean

# Via AUR (when available)
yay -S oxidclean
```

### Quick Start

```bash
# Scan system and view report
oxidclean scan

# List orphan packages
oxidclean list

# Remove orphans interactively
sudo oxidclean clean

# Analyze a specific package
oxidclean analyze vim

# View cache statistics
oxidclean cache --stats

# Clean package cache
sudo oxidclean cache --clean
```

### Commands

| Command | Description |
|---------|-------------|
| `scan` | Scans the system and displays orphan package report |
| `clean` | Removes orphan packages interactively |
| `analyze <pkg>` | Analyzes dependencies of a specific package |
| `cache` | Manages pacman cache |
| `list` | Lists all orphan packages |

### Documentation

- [Installation](docs/en/INSTALL.md)
- [Usage Guide](docs/en/USAGE.md)
- [Troubleshooting](docs/en/TROUBLESHOOTING.md)

---

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Benchmarks

```bash
cargo bench
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or pull request.
