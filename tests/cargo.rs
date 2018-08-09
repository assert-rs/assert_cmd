extern crate assert_cmd;
extern crate escargot;
extern crate predicates;

use std::process;

use assert_cmd::prelude::*;

#[test]
fn main_binary() {
    let mut cmd = process::Command::main_binary().unwrap();
    cmd.env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn main_binary_with_empty_env() {
    let mut cmd = process::Command::main_binary().unwrap();
    cmd.env_clear().env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn cargo_binary() {
    let mut cmd = process::Command::cargo_bin("bin_fixture").unwrap();
    cmd.env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn cargo_binary_with_empty_env() {
    let mut cmd = process::Command::cargo_bin("bin_fixture").unwrap();
    cmd.env_clear().env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn cargo_example() {
    let mut cmd = process::Command::cargo_example("example_fixture").unwrap();
    cmd.env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn cargo_example_with_empty_env() {
    let mut cmd = process::Command::cargo_example("example_fixture").unwrap();
    cmd.env_clear().env("stdout", "42");
    cmd.assert().success().stdout("42\n");
}

#[test]
fn cargo_example_cache() {
    let bin_under_test = escargot::CargoBuild::new()
        .bin("bin_fixture")
        .current_release()
        .current_target()
        .run()
        .unwrap();
    bin_under_test.command().unwrap();
}
