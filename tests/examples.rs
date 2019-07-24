extern crate assert_cmd;
extern crate escargot;
extern crate predicates;

use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn lib_example() {
    let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
    cmd.arg("-A").env("stdout", "hello").env("exit", "42");
    let assert = cmd.with_stdin().buffer("42").assert();
    assert.failure().code(42).stdout("hello\n");
}
