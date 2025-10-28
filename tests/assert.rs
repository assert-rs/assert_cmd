use std::process::Command;

use assert_cmd::cargo_bin;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn stdout_string() {
    let expected = "hello\n".to_owned();
    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout(expected);
}

#[test]
fn trait_example() {
    let mut cmd = Command::new(cargo_bin!("bin_fixture"));
    cmd.assert().success();
}

#[test]
fn trait_assert_example() {
    let mut cmd = Command::new(cargo_bin!("bin_fixture"));
    cmd.assert().success();
}

#[test]
fn struct_example() {
    let mut cmd = Command::new(cargo_bin!("bin_fixture"));
    cmd.assert().success();
}

#[test]
fn append_context_example() {
    Command::new(cargo_bin!("bin_fixture"))
        .assert()
        .append_context("main", "no args")
        .success();
}

#[test]
fn success_example() {
    Command::new(cargo_bin!("bin_fixture")).assert().success();
}

#[test]
fn failure_example() {
    Command::new(cargo_bin!("bin_fixture"))
        .env("exit", "1")
        .assert()
        .failure();
}

#[test]
fn code_example() {
    Command::new(cargo_bin!("bin_fixture"))
        .env("exit", "42")
        .assert()
        .code(predicate::eq(42));

    Command::new(cargo_bin!("bin_fixture"))
        .env("exit", "42")
        .assert()
        .code(42);

    Command::new(cargo_bin!("bin_fixture"))
        .env("exit", "42")
        .assert()
        .code(&[2, 42] as &[i32]);
}

#[test]
fn stdout_example() {
    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout(predicate::eq(b"hello\n" as &[u8]));

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout(predicate::str::diff("hello\n"));

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout(b"hello\n" as &[u8]);

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout(vec![b'h', b'e', b'l', b'l', b'o', b'\n']);

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout("hello\n");
}

#[test]
fn stderr_example() {
    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr(predicate::eq(b"world\n" as &[u8]));

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr(predicate::str::diff("world\n"));

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr(b"world\n" as &[u8]);

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr(vec![b'w', b'o', b'r', b'l', b'd', b'\n']);

    Command::new(cargo_bin!("bin_fixture"))
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr("world\n");
}
