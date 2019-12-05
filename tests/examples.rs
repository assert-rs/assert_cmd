use assert_cmd::Command;

#[test]
fn lib_example() {
    let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
    let assert = cmd
        .arg("-A")
        .env("stdout", "hello")
        .env("exit", "42")
        .write_stdin("42")
        .assert();
    assert.failure().code(42).stdout("hello\n");
}
