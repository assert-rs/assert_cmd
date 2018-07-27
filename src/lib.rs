//! **Assert `process::Command`** - Easy command initialization and assertions.
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.6"
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
//! * [duct][duct] for orchestrating multiple processes.
//! * [`assert_fs`][assert_fs] for filesystem fixtures and assertions.
//! * [dir-diff][dir-diff] for testing file side-effects.
//! * [tempfile][tempfile] for scratchpad directories.
//!
//! [dir-diff]: https://crates.io/crates/dir-diff
//! [tempfile]: https://crates.io/crates/tempfile
//! [duct]: https://crates.io/crates/duct
//! [assert_fs]: https://crates.io/crates/assert_fs
//!
//! ## Migrating from `assert_cli` v0.6
//!
//! `assert_cli` is the successor to `assert_cli`:
//! - More flexible, reusable assertions (also used by `assert_fs`).
//! - Can integrate with other process-management crates, like `duct`.
//! - Addresses several architectural problems.
//!
//! Key points in migrating from `assert_cli`:
//! - `std::prcoess::Command` is extended with traits rather than being wrapping in custom logic.
//! - The command-under-test is run eagerly, with assertions happening immediately.
//! - `success()` is not implicit and requires being explicitly called.
//! - `stdout`/`stderr` aren't automatically trimmed before being passed to the `Predicate`.

#![warn(missing_docs)]

extern crate escargot;
extern crate predicates;
#[macro_use]
extern crate serde;

pub mod assert;
pub use assert::Assert;
pub use assert::OutputAssertExt;
pub mod cargo;
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
