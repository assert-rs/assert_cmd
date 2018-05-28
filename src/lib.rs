//! **Assert `process::Command`** - Easy command initialization and assertions.
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.1"
//! ```

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
mod cargo;
pub use cargo::*;
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
