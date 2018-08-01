extern crate assert_cmd;
extern crate predicates;

use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn code_example() {
    Command::main_binary()
        .unwrap()
        .env("exit", "42")
        .assert()
        .code(predicate::eq(42));

    // which can be shortened to:
    Command::main_binary()
        .unwrap()
        .env("exit", "42")
        .assert()
        .code(42);
}

#[test]
fn stdout_example() {
    Command::main_binary()
        .unwrap()
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout(predicate::str::similar("hello\n").from_utf8());

    // which can be shortened to:
    Command::main_binary()
        .unwrap()
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stdout("hello\n");
}

#[test]
fn stderr_example() {
    Command::main_binary()
        .unwrap()
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr(predicate::str::similar("world\n").from_utf8());

    // which can be shortened to:
    Command::main_binary()
        .unwrap()
        .env("stdout", "hello")
        .env("stderr", "world")
        .assert()
        .stderr("world\n");
}
