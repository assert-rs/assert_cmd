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

use std::ffi;
use std::path;
use std::process;

use escargot;
use failure;

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
    fn main_binary() -> Result<Self, failure::Error>;

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
    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, failure::Error>;

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
    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, failure::Error>;
}

impl CommandCargoExt for process::Command {
    fn main_binary() -> Result<Self, failure::Error> {
        let cmd = main_binary_path()?;
        Ok(process::Command::new(&cmd))
    }

    fn cargo_bin<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, failure::Error> {
        let cmd = cargo_bin_path(name)?;
        Ok(process::Command::new(&cmd))
    }

    fn cargo_example<S: AsRef<ffi::OsStr>>(name: S) -> Result<Self, failure::Error> {
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
pub fn main_binary_path() -> Result<path::PathBuf, failure::Error> {
    let cargo = escargot::Cargo::new().build().current_release();
    let bins: Vec<_> = cargo
        .exec()?
        .filter_map(|m| extract_filenames(&m, "bin"))
        .collect();
    if bins.is_empty() {
        bail!("No binaries in crate");
    } else if bins.len() != 1 {
        bail!("Ambiguous which binary is intended: {:?}", bins);
    }
    Ok(bins.into_iter().next().expect("already validated"))
}

/// Get the path to the specified binary of the current crate.
pub fn cargo_bin_path<S: AsRef<ffi::OsStr>>(name: S) -> Result<path::PathBuf, failure::Error> {
    let cargo = escargot::Cargo::new().build().bin(name).current_release();
    let bins: Vec<_> = cargo
        .exec()?
        .filter_map(|m| extract_filenames(&m, "bin"))
        .collect();
    assert_eq!(bins.len(), 1);
    Ok(bins.into_iter().next().expect("already validated"))
}

/// Get the path to the specified example of the current crate.
pub fn cargo_example_path<S: AsRef<ffi::OsStr>>(name: S) -> Result<path::PathBuf, failure::Error> {
    let cargo = escargot::Cargo::new()
        .build()
        .example(name)
        .current_release();
    let bins: Vec<_> = cargo
        .exec()?
        .filter_map(|m| extract_filenames(&m, "example"))
        .collect();
    assert_eq!(bins.len(), 1);
    Ok(bins.into_iter().next().expect("already validated"))
}
