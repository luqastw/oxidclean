# Guia de Uso

## Comandos Disponíveis

### scan - Escanear Sistema

Analisa o sistema e identifica pacotes órfãos.

```bash
# Scan básico
oxidclean scan

# Ordenar por tamanho
oxidclean scan --sort-by-size

# Saída em JSON
oxidclean scan --json

# Exportar para arquivo
oxidclean scan --export relatorio.json
```

### clean - Remover Pacotes

Remove pacotes órfãos de forma interativa.

```bash
# Modo interativo (pede confirmação para cada pacote)
sudo oxidclean clean

# Confirmar todas as remoções automaticamente
sudo oxidclean clean --yes

# Simular remoções (não executa)
oxidclean clean --dry-run
```

### analyze - Analisar Pacote

Analisa as dependências de um pacote específico.

```bash
# Analisar vim
oxidclean analyze vim

# Saída em JSON
oxidclean analyze vim --json
```

Mostra:
- Dependências diretas
- Dependências transitivas
- Pacotes que dependem dele (reverse dependencies)
- Tamanho total com dependências

### cache - Gerenciar Cache

Gerencia o cache de pacotes do pacman.

```bash
# Ver estatísticas do cache
oxidclean cache --stats

# Limpar versões antigas (mantém 3 por padrão)
sudo oxidclean cache --clean

# Limpar mantendo apenas 1 versão
sudo oxidclean cache --clean --keep 1

# Simular limpeza
oxidclean cache --clean --dry-run

# Limpar sem confirmação
sudo oxidclean cache --clean --yes
```

### list - Listar Órfãos

Lista todos os pacotes órfãos (modo simples).

```bash
# Lista básica
oxidclean list

# Ordenar por tamanho
oxidclean list --sort-by-size
```

## Flags Globais

| Flag | Descrição |
|------|-----------|
| `-v, --verbose` | Modo verboso - exibe mais detalhes |
| `-q, --quiet` | Modo silencioso - apenas resultados |
| `-h, --help` | Mostra ajuda |
| `-V, --version` | Mostra versão |

## Exemplos Práticos

### Workflow básico de limpeza

```bash
# 1. Ver relatório do sistema
oxidclean scan

# 2. Revisar órfãos (modo dry-run)
oxidclean clean --dry-run

# 3. Remover órfãos interativamente
sudo oxidclean clean

# 4. Limpar cache
sudo oxidclean cache --clean
```

### Automatização com scripts

```bash
# Listar órfãos (para uso em scripts)
oxidclean list --quiet

# Remover todos órfãos automaticamente
sudo oxidclean clean --yes

# Exportar relatório JSON para processamento
oxidclean scan --json > /tmp/report.json
```

### Análise de um pacote antes de remover

```bash
# Ver o que depende de um pacote
oxidclean analyze libcups

# Se nada depender, é seguro remover
# Se algo depender, aparecerá em "reverse_dependencies"
```

## Níveis de Risco

O OxidClean classifica pacotes em três níveis:

| Nível | Cor | Descrição |
|-------|-----|-----------|
| Safe | Verde | Seguro para remover |
| Caution | Amarelo | Revisar antes de remover (relacionado a sistema) |
| Critical | Vermelho | Nunca remover (pacotes protegidos) |

## Pacotes Protegidos

Por padrão, os seguintes pacotes são protegidos:

- `base`, `linux`, `linux-lts`, `linux-zen`, `linux-hardened`
- `linux-firmware`, `pacman`, `systemd`
- `glibc`, `coreutils`, `bash`, `shadow`, `util-linux`

Você pode adicionar mais pacotes no arquivo de configuração.
