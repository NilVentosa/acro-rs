use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

const PRG: &str = "acro";
const EMPTY: &str = "tests/inputs/empty.csv";
const DEFAULT_COLUMNS_FILE: &str = "tests/inputs/def.csv";
const DIFFERENT_COLUMNS_FILE: &str = "tests/inputs/diff.csv";

const NATO_DESC: &str = "North Atlantic Treaty Organization";
const NATO_ACR: &str = "NAto";

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run_acro(&[NATO_ACR, "-f", EMPTY], String::from(""))
}

#[test]
fn default_columns() -> TestResult {
    run_acro(
        &[NATO_ACR, "-f", DEFAULT_COLUMNS_FILE],
        String::from(NATO_DESC),
    )
}

#[test]
fn different_columns() -> TestResult {
    run_acro(
        &[NATO_ACR, "-f", DIFFERENT_COLUMNS_FILE, "-a 2 -d 3"],
        String::from(NATO_DESC),
    )
}

#[test]
fn all_arguments() -> TestResult {
    run_acro(
        &[NATO_ACR, "-f", DEFAULT_COLUMNS_FILE, "-a 1 -d 2"],
        String::from(NATO_DESC),
    )
}

fn run_acro(args: &[&str], expected: String) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}