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
        .code(42);
    // which is equivalent to
    Command::main_binary()
        .unwrap()
        .env("exit", "42")
        .assert()
        .code(predicate::eq(42));
}
