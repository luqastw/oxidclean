# Instalação

## Requisitos

- Arch Linux ou derivado (Manjaro, EndeavourOS, etc.)
- Rust 1.70+ (para compilação)
- Permissões de root (para remoção de pacotes)

## Métodos de Instalação

### Via Cargo (Recomendado para desenvolvedores)

```bash
cargo install oxidclean
```

### Via AUR (Arch User Repository)

```bash
# Com yay
yay -S oxidclean

# Com paru
paru -S oxidclean

# Manualmente
git clone https://aur.archlinux.org/oxidclean.git
cd oxidclean
makepkg -si
```

### Compilação Manual

```bash
# Clonar repositório
git clone https://github.com/luqastw/oxidclean.git
cd oxidclean

# Compilar release
cargo build --release

# Instalar (opcional)
sudo cp target/release/oxidclean /usr/local/bin/
```

## Verificação

Após instalação, verifique se está funcionando:

```bash
oxidclean --version
oxidclean --help
```

## Configuração Inicial

O OxidClean funciona sem configuração, mas você pode criar um arquivo de configuração:

```bash
mkdir -p ~/.config/oxidclean
```

Crie `~/.config/oxidclean/config.toml`:

```toml
# Pacotes a ignorar na análise
ignored_packages = []

# Pacotes protegidos (nunca serão removidos)
# Os padrões já incluem: base, linux, pacman, systemd, glibc, etc.
protected_packages = []

# Modo interativo por padrão
interactive = true

# Versões de cache a manter por pacote
cache_keep_versions = 3
```

## Desinstalação

### Via Cargo

```bash
cargo uninstall oxidclean
```

### Via AUR

```bash
sudo pacman -Rns oxidclean
```

### Manual

```bash
sudo rm /usr/local/bin/oxidclean
rm -rf ~/.config/oxidclean
```
