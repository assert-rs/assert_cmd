use std::process::Command;

use assert_cmd::cargo_bin;
use assert_cmd::prelude::*;
use escargot::CURRENT_TARGET;

#[test]
fn cargo_binary() {
    let mut cmd = Command::new(cargo_bin!("bin_fixture"));
    cmd.env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn cargo_binary_with_empty_env() {
    let mut cmd = Command::new(cargo_bin!("bin_fixture"));
    cmd.env_clear().env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn mod_example() {
    let runner_env = format!(
        "CARGO_TARGET_{}_RUNNER",
        CURRENT_TARGET.replace('-', "_").to_uppercase()
    );
    if std::env::var(runner_env).is_ok() {
        // not running this test on cross because escargot doesn't support the cargo target runner yet
    } else {
        let bin_under_test = escargot::CargoBuild::new()
            .bin("bin_fixture")
            .current_release()
            .current_target()
            .run()
            .unwrap();
        let mut cmd = bin_under_test.command();
        let output = cmd.unwrap();
        println!("{output:?}");
    }
}

#[test]
fn cargo_bin_example_2() {
    let mut cmd = Command::new(cargo_bin!("bin_fixture"));
    let output = cmd.unwrap();
    println!("{output:?}");
}
