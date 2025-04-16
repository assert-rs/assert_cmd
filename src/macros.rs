/// Allows you to pull the name from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```should_panic
/// use assert_cmd::Command;
///
/// let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
/// let assert = cmd
///     .arg("-A")
///     .env("stdout", "hello")
///     .env("exit", "42")
///     .write_stdin("42")
///     .assert();
/// assert
///     .failure()
///     .code(42)
///     .stdout("hello\n");
/// ```
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}
