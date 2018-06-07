//! **Assert `process::Command`** - Easy command initialization and assertions.
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.2"
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

#![warn(missing_docs)]

extern crate escargot;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate predicates;
#[macro_use]
extern crate serde;

mod assert;
pub use assert::*;
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
