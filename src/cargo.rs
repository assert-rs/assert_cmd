//! Simplify running `bin`s in a Cargo project.
//!
//! [`CommandCargoExt`] is an extension trait for [`Command`] to easily launch a crate's
//! binaries.
//!
//! # Examples
//!
//! Simple case:
//!
//! ```rust,no_run
//! use assert_cmd::prelude::*;
//!
//! use std::process::Command;
//!
//! let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
//!     .unwrap();
//! let output = cmd.unwrap();
//! ```
//!
//! # Limitations
//!
//! - Only works within the context of integration tests.  See [`escargot`] for a more
//!   flexible API.
//! - Only reuses your existing feature flags, targets, or build mode.
//! - Only works with cargo binaries (`cargo test` ensures they are built).
//!
//! If you run into these limitations, we recommend trying out [`escargot`]:
//!
//! ```rust,no_run
//! use assert_cmd::prelude::*;
//!
//! use std::process::Command;
//!
//! let bin_under_test = escargot::CargoBuild::new()
//!     .bin("bin_fixture")
//!     .current_release()
//!     .current_target()
//!     .run()
//!     .unwrap();
//! let mut cmd = bin_under_test.command();
//! let output = cmd.unwrap();
//! println!("{:?}", output);
//! ```
//!
//! Notes:
//! - There is a [noticeable per-call overhead](cargo-overhead) for `CargoBuild`.  We recommend
//!   caching the binary location (`.path()` instead of `.command()`) with [`lazy_static`].
//! - `.current_target()` improves platform coverage at the cost of [slower test runs if you don't
//!   explicitly pass `--target <TRIPLET>` on the command line](first-call).
//!
//! [`lazy_static`]: https://crates.io/crates/lazy_static
//! [`CommandCargoExt`]: trait.CommandCargoExt.html
//! [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
//! [`escargot`]: https://docs.rs/escargot/
//! [cargo-overhead]: https://github.com/assert-rs/assert_cmd/issues/6
//! [first-call]: https://github.com/assert-rs/assert_cmd/issues/57

use std::env;
use std::error::Error;
use std::fmt;
use std::path;
use std::process;

/// Create a [`Command`] for a `bin` in the Cargo project.
///
/// `CommandCargoExt` is an extension trait for [`Command`][Command] to easily launch a crate's
/// binaries.
///
/// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
///
/// # Examples
///
/// ```rust,no_run
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
///     .unwrap();
/// let output = cmd.unwrap();
/// println!("{:?}", output);
/// ```
///
/// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
/// [`cargo`]: index.html
pub trait CommandCargoExt
where
    Self: Sized,
{
    /// Create a [`Command`] to run a specific binary of the current crate.
    ///
    /// See the [`cargo` module documentation][`cargo`] for caveats and workarounds.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// println!("{:?}", output);
    /// ```
    ///
    /// ```rust,no_run
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::cargo_bin("bin_fixture")
    ///     .unwrap();
    /// let output = cmd.unwrap();
    /// println!("{:?}", output);
    /// ```
    ///
    /// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [`cargo`]: index.html
    fn cargo_bin<S: AsRef<str>>(name: S) -> Result<Self, CargoError>;
}

impl CommandCargoExt for crate::cmd::Command {
    fn cargo_bin<S: AsRef<str>>(name: S) -> Result<Self, CargoError> {
        crate::cmd::Command::cargo_bin(name)
    }
}

impl CommandCargoExt for process::Command {
    fn cargo_bin<S: AsRef<str>>(name: S) -> Result<Self, CargoError> {
        cargo_bin_cmd(name)
    }
}

pub(crate) fn cargo_bin_cmd<S: AsRef<str>>(name: S) -> Result<process::Command, CargoError> {
    let path = cargo_bin(name);
    if path.is_file() {
        Ok(process::Command::new(path))
    } else {
        Err(CargoError::with_cause(NotFoundError { path }))
    }
}

/// Error when finding crate binary.
#[derive(Debug)]
pub struct CargoError {
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl CargoError {
    /// Wrap the underlying error for passing up.
    pub fn with_cause<E>(cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let cause = Box::new(cause);
        Self { cause: Some(cause) }
    }
}

impl Error for CargoError {}

impl fmt::Display for CargoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref cause) = self.cause {
            writeln!(f, "Cause: {}", cause)?;
        }
        Ok(())
    }
}

/// Error when finding crate binary.
#[derive(Debug)]
struct NotFoundError {
    path: path::PathBuf,
}

impl Error for NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cargo command not found: {}", self.path.display())
    }
}

// Adapted from
// https://github.com/rust-lang/cargo/blob/485670b3983b52289a2f353d589c57fae2f60f82/tests/testsuite/support/mod.rs#L507
fn target_dir() -> path::PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap()
}

/// Look up the path to a cargo-built binary within an integration test.
pub fn cargo_bin<S: AsRef<str>>(name: S) -> path::PathBuf {
    cargo_bin_str(name.as_ref())
}

fn cargo_bin_str(name: &str) -> path::PathBuf {
    target_dir().join(format!("{}{}", name, env::consts::EXE_SUFFIX))
}
