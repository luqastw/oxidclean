#!/bin/bash
# Script para gerar relatório de code coverage

set -e

COVERAGE_DIR="target/coverage"

echo "=== OxidClean Code Coverage ==="
echo

# Verificar se tarpaulin está instalado
if command -v cargo-tarpaulin &> /dev/null; then
    echo "Usando cargo-tarpaulin..."
    cargo tarpaulin --out Html --out Xml --output-dir "$COVERAGE_DIR" --ignore-tests
    echo
    echo "Relatório HTML: $COVERAGE_DIR/tarpaulin-report.html"
    echo "Relatório XML:  $COVERAGE_DIR/cobertura.xml"

# Verificar se llvm-cov está instalado
elif command -v cargo-llvm-cov &> /dev/null; then
    echo "Usando cargo-llvm-cov..."
    cargo llvm-cov --html --output-dir "$COVERAGE_DIR"
    echo
    echo "Relatório: $COVERAGE_DIR/html/index.html"

# Fallback: instruções
else
    echo "Nenhuma ferramenta de coverage encontrada."
    echo
    echo "Instale uma das seguintes:"
    echo "  cargo install cargo-tarpaulin"
    echo "  cargo install cargo-llvm-cov"
    echo
    echo "Ou use grcov manualmente (veja docs/COVERAGE.md)"
    exit 1
fi

echo
echo "=== Coverage gerado com sucesso! ==="
