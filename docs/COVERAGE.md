# Code Coverage para OxidClean

## Opção 1: Usando cargo-tarpaulin (recomendado)

```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Rodar coverage
cargo tarpaulin --out Html --output-dir coverage/

# Abrir relatório
xdg-open coverage/tarpaulin-report.html
```

## Opção 2: Usando cargo-llvm-cov

```bash
# Instalar llvm-cov
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# Rodar coverage
cargo llvm-cov --html --output-dir coverage/

# Abrir relatório
xdg-open coverage/html/index.html
```

## Opção 3: Usando grcov (manual)

```bash
# Instalar grcov
cargo install grcov

# Compilar com instrumentação
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

# Gerar relatório
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/

# Abrir relatório
xdg-open target/coverage/index.html
```

## Meta de Cobertura

- **Mínimo**: 70%
- **Ideal**: 80%+

## Módulos Prioritários para Cobertura

1. `src/core/` - Lógica principal
2. `src/dal/` - Acesso a dados
3. `src/models/` - Modelos de dados

## CI/CD Integration

Para GitHub Actions, adicione ao workflow:

```yaml
- name: Install tarpaulin
  run: cargo install cargo-tarpaulin

- name: Run coverage
  run: cargo tarpaulin --out Xml --output-dir coverage/

- name: Upload coverage
  uses: codecov/codecov-action@v3
  with:
    files: coverage/cobertura.xml
```
