use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn lib_example() {
    let mut cmd = cargo_bin_cmd!("bin_fixture");
    cmd.assert().success();

    let mut cmd = cargo_bin_cmd!("bin_fixture");
    let assert = cmd
        .arg("-A")
        .env("stdout", "hello")
        .env("exit", "42")
        .write_stdin("42")
        .assert();
    assert.failure().code(42).stdout("hello\n");
}

#[test]
fn timeout_example() {
    let assert = cargo_bin_cmd!("bin_fixture")
        .timeout(std::time::Duration::from_secs(1))
        .env("sleep", "100")
        .assert();
    assert.failure();
}
