use std::{env, time::Duration};

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn fails_on_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd
        .assert()
        .failure()
        .stderr(predicate::str::contains("No secret number"));

    Ok(())
}

#[test]
fn fails_on_non_number_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("non_number");
    cmd
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a number"));

    Ok(())
}

#[test]
fn valid_guess_ends_with_win() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd
        .arg("55")
        .write_stdin("55");
    cmd
        .assert()
        .stdout(predicates::str::contains("win"))
        .success();

    Ok(())
}

#[test]
fn greater_guess_reported() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd
        .arg("55")
        .write_stdin("60")
        .timeout(Duration::from_millis(100));
    cmd
        .assert()
        .stdout(predicates::str::contains("big"))
        .failure();

    Ok(())
}

#[test]
fn lower_guess_reported() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd
        .arg("55")
        .write_stdin("50")
        .timeout(Duration::from_millis(100));
    cmd
        .assert()
        .stdout(predicates::str::contains("small"))
        .failure();

    Ok(())
}