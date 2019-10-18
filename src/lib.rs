//! **Assert [`Command`]** - Easy command initialization and assertions.
//!
//! `assert_cmd` includes support for:
//! - Setting up your program-under-test (see [`CommandCargoExt`], [`CommandStdInExt`]).
//! - Verifying your program-under-test (see [`OutputOkExt`], [`OutputAssertExt`]).
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.11"
//! ```
//!
//! ## Overview
//!
//! Create a [`Command`]:
//! - `Command::new(path)`, see [`Command`]
//! - `Command::cargo_bin(name)`, see [`CommandCargoExt`]
//!
//! Configure a [`Command`]:
//! - `arg` / `args`, see [`Command`]
//! - `current_dir`, see [`Command`]
//! - `env` / `envs` / `env_remove` / `env_clear`, see [`Command`]
//! - `with_stdin`, see [`CommandStdInExt`]
//!
//! Validate either a [`Command`] or `Output`:
//! - `ok` / `unwrap` / `unwrap_err`, see [`OutputOkExt`]
//! - `assert` ([`OutputAssertExt`])
//!   - `success`, see [`Assert`]
//!   - `failure`, see [`Assert`]
//!   - `interrupted`, see [`Assert`]
//!   - `code`, see [`Assert`]
//!   - `stdout`, see [`Assert`]
//!   - `stderr`, see [`Assert`]
//!
//! ## Examples
//!
//! Here's a trivial example:
//! ```rust,no_run
//! extern crate assert_cmd;
//!
//! use std::process::Command;
//! use assert_cmd::prelude::*;
//!
//! fn main() {
//!     let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
//!     cmd.assert().success();
//! }
//! ```
//!
//! And a little of everything:
//! ```rust,no_run
//! extern crate assert_cmd;
//!
//! use std::process::Command;
//! use assert_cmd::prelude::*;
//!
//! fn main() {
//!     let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
//!     cmd
//!         .arg("-A")
//!         .env("stdout", "hello")
//!         .env("exit", "42");
//!     let assert = cmd
//!         .with_stdin()
//!         .buffer("42")
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
//! - [`Command`] is extended with traits rather than being wrapping in custom logic.
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
//! [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
//! [`Assert`]: assert/struct.Assert.html
//! [`success()`]: assert/struct.Assert.html#method.success
//! [`CommandCargoExt`]: cargo/trait.CommandCargoExt.html
//! [`CommandStdInExt`]: stdin/trait.CommandStdInExt.html
//! [`OutputOkExt`]: cmd/trait.OutputOkExt.html
//! [`OutputAssertExt`]: assert/trait.OutputAssertExt.html

#![warn(missing_docs)]

/// Allows you to pull the name from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```should_panic
/// #[macro_use]
/// extern crate assert_cmd;
///
/// use std::process::Command;
/// use assert_cmd::prelude::*;
///
/// fn main() {
///     let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
///     cmd
///         .arg("-A")
///         .env("stdout", "hello")
///         .env("exit", "42")
///         .with_stdin()
///         .buffer("42");
///     let assert = cmd.assert();
///     assert
///         .failure()
///         .code(42)
///         .stdout("hello\n");
/// }
/// ```
#[cfg(not(feature = "no_cargo"))]
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

pub mod assert;
pub mod cargo;
pub mod cmd;
pub mod stdin;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use crate::assert::OutputAssertExt;
    pub use crate::cargo::CommandCargoExt;
    pub use crate::cmd::OutputOkExt;
    pub use crate::stdin::CommandStdInExt;
}

#[macro_use]
extern crate doc_comment;
doctest!("../README.md");
