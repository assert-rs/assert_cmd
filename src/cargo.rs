//! Simplify running `bin`s in a Cargo project.
//!
//! [`CommandCargoExt`] is an extension trait for [`Command`] to easily launch a crate's
//! binaries.
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
//! let mut cmd = Command::main_binary()
//!     .unwrap();
//! let output = cmd.unwrap();
//! ```
//!
//! # Further customizations
//!
//! There are times when you might want to drop down to the underlying API, [`escargot`]:
//! - Specifying feature flags
//! - Workaround the [per-call cargo overhead][cargo-overhead] by caching the binary location with [`lazy_static`].
//! - [If not using `--target <TRIPLET>`, bypass the first call overhead][first-call] by not
//!   passing `current_target()` to [`escargot`].
//! - Using bin or example from another crate from workspaces
//!
//! This can be done by using [`CommandCargoBuildExt`] trait.
//!
//! ```rust
//! extern crate assert_cmd;
//!
//! use assert_cmd::prelude::*;
//!
//! use std::process::Command;
//!
//! let mut bin_under_test = Command::cargo_builder()
//!     .bin("bin_fixture")
//!     .current_release()
//!     .current_target()
//!     .build_command()
//!     .unwrap();
//! let output = bin_under_test.unwrap();
//! ```
//!
//! [`lazy_static`]: https://crates.io/crates/lazy_static
//! [`CommandCargoExt`]: trait.CommandCargoExt.html
//! [`CommandCargoBuildExt`]: trait.CommandCargoBuildExt.html
//! [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
//! [`escargot`]: https://docs.rs/escargot/
//! [cargo-overhead]: https://github.com/assert-rs/assert_cmd/issues/6
//! [first-call]: https://github.com/assert-rs/assert_cmd/issues/57

use std::error::Error;
use std::ffi;
use std::fmt;
use std::process;

use escargot;

/// `CommandCargoBuildExt` is an extension trait for [`CargoBuild`][CargoBuild] to run
/// command using CargoBuild builder
///
/// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let mut cmd = Command::cargo_builder()
///     .bin("bin_fixture")
///     .package("assert_cmd")
///     .current_release()
///     .current_target()
///     .build_command()
///     .unwrap();
/// let output = cmd.unwrap();
/// ```
///
/// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
/// [`cargo`]: index.html
pub trait CommandCargoBuildExt
where
    Self: Sized,
{
    /// Create a [`Command`] using [`CargoBuild`] builder.
    ///
    /// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_builder()
    ///     .example("example_fixture")
    ///     .package("assert_cmd")
    ///     .current_release()
    ///     .current_target()
    ///     .build_command()
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [`CargoBuild`]: https://docs.rs/escargot/0.4.0/escargot/struct.CargoBuild.html
    /// [`cargo`]: index.html
    fn build_command(self) -> Result<process::Command, CargoError>;
}

impl CommandCargoBuildExt for escargot::CargoBuild {
    fn build_command(self) -> Result<process::Command, CargoError> {
        let runner = self.run().map_err(CargoError::with_cause)?;
        Ok(runner.command())
    }
}

/// Create a [`Command`] for a `bin` in the Cargo project.
///
/// `CommandCargoExt` is an extension trait for [`Command`][Command] to easily launch a crate's
/// binaries.
///
/// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let mut cmd = Command::main_binary()
///     .unwrap();
/// let output = cmd.unwrap();
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
    /// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::main_binary()
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [`cargo`]: index.html
    fn main_binary() -> Result<Self, CargoError>;

    /// Create a [`Command`] to run a specific binary of the current crate.
    ///
    /// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_bin("bin_fixture")
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [`cargo`]: index.html
    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError>;

    /// Create a [`Command`] to run a specific example of the current crate.
    ///
    /// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_example("example_fixture")
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [`cargo`]: index.html
    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError>;

    /// Create a [`CargoBuild`] builder to construct any cargo run command
    ///
    /// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_builder()
    ///     .bin("bin_fixture")
    ///     .package("assert_cmd")
    ///     .current_release()
    ///     .current_target()
    ///     .build_command()
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [`CargoBuild`]: https://docs.rs/escargot/0.4.0/escargot/struct.CargoBuild.html
    /// [`cargo`]: index.html
    fn cargo_builder() -> escargot::CargoBuild {
        escargot::CargoBuild::new()
    }
}

impl CommandCargoExt for process::Command {
    fn main_binary() -> Result<Self, CargoError> {
        let runner = escargot::CargoBuild::new()
            .current_release()
            .current_target()
            .run()
            .map_err(CargoError::with_cause)?;
        Ok(runner.command())
    }

    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError> {
        let runner = escargot::CargoBuild::new()
            .bin(name)
            .current_release()
            .current_target()
            .run()
            .map_err(CargoError::with_cause)?;
        Ok(runner.command())
    }

    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError> {
        let runner = escargot::CargoBuild::new()
            .example(name)
            .current_release()
            .current_target()
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
