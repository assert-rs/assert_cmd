//! **Assert [`Command`]** - Easy command initialization and assertions.
//!
//! `assert_cmd` includes support for:
//! - Setting up your program-under-test.
//! - Verifying your program-under-test.
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.12"
//! ```
//!
//! ## Overview
//!
//! Create a [`Command`]:
//! - `Command::new(path)`
//! - `Command::from_std(...)`
//! - `Command::cargo_bin(name)`
//!
//! Configure a [`Command`]:
//! - `arg` / `args`
//! - `current_dir`
//! - `env` / `envs` / `env_remove` / `env_clear`
//! - `write_stdin` / `pipe_stdin`
//!
//! Validate a [`Command`]:
//! - `ok` / `unwrap` / `unwrap_err`
//! - `assert`
//!   - `success`, see [`Assert`]
//!   - `failure`, see [`Assert`]
//!   - `interrupted`, see [`Assert`]
//!   - `code`, see [`Assert`]
//!   - `stdout`, see [`Assert`]
//!   - `stderr`, see [`Assert`]
//!
//! Note: [`Command`] is provided as a convenience. Extension traits for [`std::process::Command`]
//! and `Output` are provided for interoperability:
//! - [`CommandCargoExt`]
//! - [`OutputOkExt`]
//! - [`OutputAssertExt`]
//!
//! ## Examples
//!
//! Here's a trivial example:
//! ```rust,no_run
//! use assert_cmd::Command;
//!
//! fn main() {
//!     let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
//!     cmd.assert().success();
//! }
//! ```
//!
//! And a little of everything:
//! ```rust,no_run
//! use assert_cmd::Command;
//!
//! fn main() {
//!     let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
//!     let assert = cmd
//!         .arg("-A")
//!         .env("stdout", "hello")
//!         .env("exit", "42")
//!         .write_stdin("42")
//!         .assert();
//!     assert
//!         .failure()
//!         .code(42)
//!         .stdout("hello\n");
//! }
//! ```
//!
//! ## Relevant crates
//!
//! Other crates that might be useful in testing command line programs.
//! - [duct] for orchestrating multiple processes.
//! - [commandspec] for easier writing of commands
//! - [assert_fs] for filesystem fixtures and assertions.
//! - [dir-diff] for testing file side-effects.
//! - [tempfile] for scratchpad directories.
//!
//! ## Migrating from `assert_cli` v0.6
//!
//! `assert_cmd` is the successor to [the original `assert_cli`][assert_cli]:
//! - More flexible, reusable assertions (also used by [assert_fs]).
//! - Can integrate with other process-management crates, like `duct`.
//! - Addresses several architectural problems.
//!
//! Key points in migrating from `assert_cli`:
//! - The command-under-test is run eagerly, with assertions happening immediately.
//! - [`success()`] is not implicit and requires being explicitly called.
//! - `stdout`/`stderr` aren't automatically trimmed before being passed to the `Predicate`.
//!
//! [commandspec]: https://crates.io/crates/commandspec
//! [assert_cli]: https://crates.io/crates/assert_cli/0.6.3
//! [dir-diff]: https://crates.io/crates/dir-diff
//! [tempfile]: https://crates.io/crates/tempfile
//! [duct]: https://crates.io/crates/duct
//! [assert_fs]: https://crates.io/crates/assert_fs
//! [`Command`]: cmd/struct.Command.html
//! [`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
//! [`Assert`]: assert/struct.Assert.html
//! [`success()`]: assert/struct.Assert.html#method.success
//! [`CommandCargoExt`]: cargo/trait.CommandCargoExt.html
//! [`OutputOkExt`]: cmd/trait.OutputOkExt.html
//! [`OutputAssertExt`]: assert/trait.OutputAssertExt.html

#![warn(missing_docs)]

/// Allows you to pull the name from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```should_panic
/// use assert_cmd::Command;
///
/// fn main() {
///     let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
///     let assert = cmd
///         .arg("-A")
///         .env("stdout", "hello")
///         .env("exit", "42")
///         .write_stdin("42")
///         .assert();
///     assert
///         .failure()
///         .code(42)
///         .stdout("hello\n");
/// }
/// ```
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

pub mod assert;
pub mod cargo;
pub mod cmd;
pub mod output;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use crate::assert::OutputAssertExt;
    pub use crate::cargo::CommandCargoExt;
    pub use crate::output::OutputOkExt;
}

pub use crate::cmd::Command;

doc_comment::doctest!("../README.md");
