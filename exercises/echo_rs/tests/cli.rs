use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

// --------------------------------------------------
#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin("echo_rs")?
        .assert()
        .failure()
         .stderr(predicate::str::contains("missing required free argument"));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echo_rs")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

// --------------------------------------------------
#[test]
fn hello1() -> Result<()> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

// --------------------------------------------------
#[test]
fn hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

// --------------------------------------------------
#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

// --------------------------------------------------
#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

// --------------------------------------------------
#[test]
fn hello1_no_delimiter() -> Result<()> {
    run(&["Hello  there", "-d"], "tests/expected/hello1.d.txt")
}

// --------------------------------------------------
#[test]
fn hello2_no_delimiter() -> Result<()> {
    run(&["-d", "Hello", "there"], "tests/expected/hello2.d.txt")
}
