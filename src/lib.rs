//! **Assert [`Command`][Command]** - Easy command initialization and assertions.
//!
//! `assert_cmd` includes support for:
//! - Setting up your program-under-test (see [`CommandCargoExt`], [`CommandStdInExt`]).
//! - Verifying your program-under-test (see [`OutputOkExt`], [`OutputAssertExt`]).
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.9"
//! ```
//!
//! ## Example
//!
//! Here's a trivial example:
//!
//! ```rust,ignore
//! extern crate assert_cmd;
//!
//! use std::process::Command;
//! use assert_cmd::prelude::*;
//!
//! fn main() {
//!     let mut cmd = Command::main_binary().unwrap();
//!     cmd.assert().success();
//! }
//! ```
//!
//! ## Relevant crates
//!
//! Other crates that might be useful in testing command line programs.
//! - [duct] for orchestrating multiple processes.
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
//! [assert_cli]: https://crates.io/crates/assert_cli/0.6.3
//! [dir-diff]: https://crates.io/crates/dir-diff
//! [tempfile]: https://crates.io/crates/tempfile
//! [duct]: https://crates.io/crates/duct
//! [assert_fs]: https://crates.io/crates/assert_fs
//! [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
//! [`success()`]: struct.Assert.html#method.success
//! [`CommandCargoExt`]: cargo/trait.CommandCargoExt.html
//! [`CommandStdInExt`]: trait.CommandStdInExt.html
//! [`OutputOkExt`]: trait.OutputOkExt.html
//! [`OutputAssertExt`]: assert/trait.OutputAssertExt.html

#![warn(missing_docs)]

extern crate escargot;
extern crate predicates;
extern crate predicates_core;
extern crate predicates_tree;

pub mod assert;
pub use assert::Assert;
pub use assert::OutputAssertExt;
pub mod cargo;
pub use cargo::CommandCargoExt;
mod cmd;
pub use cmd::*;
mod stdin;
pub use stdin::*;
mod errors;
pub use errors::*;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use assert::OutputAssertExt;
    pub use cargo::CommandCargoExt;
    pub use cmd::OutputOkExt;
    pub use stdin::CommandStdInExt;
}
