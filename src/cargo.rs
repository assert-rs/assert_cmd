//! Simplify running `bin`s in a Cargo project.
//!
//! [`CommandCargoExt`] is an extension trait for [`Command`] to easily launch a crate's
//! binaries.
//!
//! In addition, the underlying functions for looking up the crate's binaries are exposed to allow
//! for optimizations, if needed.
//!
//! # Examples
//!
//! Simple case:
//!
//! ```rust
//! use assert_cmd::prelude::*;
//!
//! use std::process::Command;
//!
//! Command::main_binary()
//!     .unwrap()
//!     .unwrap();
//! ```
//!
//! For caching to minimize cargo overhead or customize the build process, see [`escargot`].
//!
//! ```rust
//! extern crate assert_cmd;
//! extern crate escargot;
//!
//! use assert_cmd::prelude::*;
//! use escargot;
//!
//! use std::process::Command;
//!
//! let bin_under_test = escargot::CargoBuild::new()
//!     .bin("bin_fixture")
//!     .current_release()
//!     .current_target()
//!     .run()
//!     .unwrap();
//! bin_under_test.command()
//!     .unwrap();
//! ```
//!
//! Tip: Use [`lazy_static`] to cache `bin_under_test` across test functions.
//!
//! [`lazy_static`]: https://crates.io/crates/lazy_static
//! [`CommandCargoExt`]: trait.CommandCargoExt.html
//! [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
//! [`escargot`]: https://docs.rs/escargot/

use std::error::Error;
use std::ffi;
use std::fmt;
use std::process;

use escargot;

/// Create a [`Command`] for a `bin` in the Cargo project.
///
/// `CommandCargoExt` is an extension trait for [`Command`][Command] to easily launch a crate's
/// binaries.
///
/// If the cargo overhead is too high per-call, you can cache the bin's location.  See the
/// [`cargo`] module.
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// Command::main_binary()
///     .unwrap()
///     .unwrap();
/// ```
///
/// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
/// [`cargo`]: index.html
pub trait CommandCargoExt
where
    Self: Sized,
{
    /// Create a [`Command`] to run the crate's main binary.
    ///
    /// Note: only works if there one bin in the crate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::main_binary()
    ///     .unwrap()  // get cargo binary
    ///     .unwrap(); // run it
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    fn main_binary() -> Result<Self, CargoError>;

    /// Create a [`Command`] to run a specific binary of the current crate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::cargo_bin("bin_fixture")
    ///     .unwrap()
    ///     .unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError>;

    /// Create a [`Command`] to run a specific example of the current crate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::cargo_example("example_fixture")
    ///     .unwrap()
    ///     .unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError>;
}

impl CommandCargoExt for process::Command {
    fn main_binary() -> Result<Self, CargoError> {
        let runner = escargot::CargoBuild::new()
            .current_release()
            .run()
            .map_err(CargoError::with_cause)?;
        Ok(runner.command())
    }

    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError> {
        let runner = escargot::CargoBuild::new()
            .bin(name)
            .current_release()
            .run()
            .map_err(CargoError::with_cause)?;
        Ok(runner.command())
    }

    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError> {
        let runner = escargot::CargoBuild::new()
            .example(name)
            .current_release()
            .run()
            .map_err(CargoError::with_cause)?;
        Ok(runner.command())
    }
}

/// Error when finding crate binary.
#[derive(Debug)]
pub struct CargoError {
    cause: Option<Box<Error + Send + Sync + 'static>>,
}

impl CargoError {
    fn with_cause<E>(cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let cause = Box::new(cause);
        Self { cause: Some(cause) }
    }
}

impl Error for CargoError {
    fn description(&self) -> &str {
        "Cargo command failed."
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|c| {
            let c: &Error = c.as_ref();
            c
        })
    }
}

impl fmt::Display for CargoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref cause) = self.cause {
            writeln!(f, "Cause: {}", cause)?;
        }
        Ok(())
    }
}
