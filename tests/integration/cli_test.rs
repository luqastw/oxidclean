//! Testes de integração para o comando scan

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_scan_help() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("scan").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Escanear sistema"));
}

#[test]
fn test_list_help() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("list").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Listar"));
}

#[test]
fn test_clean_help() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("clean").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Remover pacotes"));
}

#[test]
fn test_analyze_help() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("analyze").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Analisar"));
}

#[test]
fn test_cache_help() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("cache").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("cache"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("oxidclean"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("oxidclean").unwrap();
    cmd.arg("invalid-command");
    cmd.assert().failure();
}
