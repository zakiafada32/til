use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").arg("world");
    cmd.assert().success().stdout("hello world\n");
    Ok(())
}

#[test]
fn runs_omit_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").arg("world").arg("-n");
    cmd.assert().success().stdout("hello world");
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "hello2.txt")
}

#[test]
fn hello1_omit_newline() -> TestResult {
    run(&["Hello  there", "-n"], "hello1.n.txt")
}

#[test]
fn hello2_omit_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let outfile = format!("tests/expected/{}", expected_file);
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}
