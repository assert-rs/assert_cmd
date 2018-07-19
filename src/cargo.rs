//! Simplify running `bin`s in a Cargo project.
//!
//! # Examples
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

use std::error::Error;
use std::ffi;
use std::fmt;
use std::path;
use std::process;

use escargot;

/// Create a `Command` for a `bin` in the Cargo project.
pub trait CommandCargoExt
where
    Self: Sized,
{
    /// Create a `Command` to run the crate's main binary.
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
    ///     .unwrap()
    ///     .unwrap();
    /// ```
    fn main_binary() -> Result<Self, CargoError>;

    /// Create a `Command` to run a specific binary of the current crate.
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
    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError>;

    /// Create a `Command` to run a specific example of the current crate.
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
    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError>;
}

impl CommandCargoExt for process::Command {
    fn main_binary() -> Result<Self, CargoError> {
        let cmd = main_binary_path()?;
        Ok(process::Command::new(&cmd))
    }

    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError> {
        let cmd = cargo_bin_path(name)?;
        Ok(process::Command::new(&cmd))
    }

    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, CargoError> {
        let cmd = cargo_example_path(name)?;
        Ok(process::Command::new(&cmd))
    }
}

#[derive(Deserialize)]
struct MessageTarget<'a> {
    #[serde(borrow)]
    crate_types: Vec<&'a str>,
    #[serde(borrow)]
    kind: Vec<&'a str>,
}

#[derive(Deserialize)]
struct MessageFilter<'a> {
    #[serde(borrow)]
    reason: &'a str,
    target: MessageTarget<'a>,
    filenames: Vec<path::PathBuf>,
}

fn extract_filenames(msg: &escargot::Message, kind: &str) -> Option<path::PathBuf> {
    let filter: MessageFilter = msg.convert().ok()?;
    if filter.reason != "compiler-artifact"
        || filter.target.crate_types != ["bin"]
        || filter.target.kind != [kind]
    {
        None
    } else {
        Some(
            filter
                .filenames
                .into_iter()
                .next()
                .expect("files must exist"),
        )
    }
}

/// Get the path to the crate's main binary.
///
/// Note: only works if there one bin in the crate.
pub fn main_binary_path() -> Result<path::PathBuf, CargoError> {
    let cargo = escargot::Cargo::new().build().current_release();
    let bins: Vec<_> = cargo
        .exec()
        .map_err(CargoError::with_cause)?
        .filter_map(|m| extract_filenames(&m, "bin"))
        .collect();
    if bins.is_empty() {
        return Err(CargoError::with_context("No binaries in crate"));
    } else if bins.len() != 1 {
        return Err(CargoError::with_context(format!(
            "Ambiguous which binary is intended: {:?}",
            bins
        )));
    }
    Ok(bins.into_iter().next().expect("already validated"))
}

/// Get the path to the specified binary of the current crate.
pub fn cargo_bin_path<S: AsRef<ffi::OsStr>>(name: S) -> Result<path::PathBuf, CargoError> {
    let cargo = escargot::Cargo::new().build().bin(name).current_release();
    let bins: Vec<_> = cargo
        .exec()
        .map_err(CargoError::with_cause)?
        .filter_map(|m| extract_filenames(&m, "bin"))
        .collect();
    assert_eq!(bins.len(), 1);
    Ok(bins.into_iter().next().expect("already validated"))
}

/// Get the path to the specified example of the current crate.
pub fn cargo_example_path<S: AsRef<ffi::OsStr>>(name: S) -> Result<path::PathBuf, CargoError> {
    let cargo = escargot::Cargo::new()
        .build()
        .example(name)
        .current_release();
    let bins: Vec<_> = cargo
        .exec()
        .map_err(CargoError::with_cause)?
        .filter_map(|m| extract_filenames(&m, "example"))
        .collect();
    assert_eq!(bins.len(), 1);
    Ok(bins.into_iter().next().expect("already validated"))
}

/// Error when finding crate binary.
#[derive(Debug)]
pub struct CargoError {
    context: Option<String>,
    cause: Option<Box<Error + Send + Sync + 'static>>,
}

impl CargoError {
    fn with_context<S>(context: S) -> Self
    where
        S: Into<String>,
    {
        let context = context.into();
        Self {
            context: Some(context),
            cause: None,
        }
    }

    fn with_cause<E>(cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let cause = Box::new(cause);
        Self {
            context: None,
            cause: Some(cause),
        }
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
        if let Some(ref context) = self.context {
            writeln!(f, "{}", context)?;
        }
        if let Some(ref cause) = self.cause {
            writeln!(f, "Cause: {}", cause)?;
        }
        Ok(())
    }
}
