//! **Assert process::Command** - Easy command initialization and assertions.
//!
//! ```toml
//! [dependencies]
//! assert_cmd = "0.1"
//! ```

#![warn(missing_docs)]

extern crate failure;

mod cmd;
pub use cmd::*;

mod errors;

/// Extension traits that are useful to have available.
pub mod prelude {
    pub use cmd::CommandStdInExt;
    pub use cmd::OutputOkExt;
    pub use cmd::OutputAssertExt;
}
