use std::process;
use std::str;

use errors::OutputError;
use errors::OutputResult;

/// Extends `std::process::Output` with methods to convert it to an `OutputResult`.
pub trait OutputOkExt
where
    Self: ::std::marker::Sized,
{
    /// Convert an `std::process::Output` into an `OutputResult`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::new("echo")
    ///     .args(&["42"])
    ///     .ok()
    ///     .unwrap();
    /// ```
    fn ok(self) -> OutputResult;

    /// Unwrap a `std::process::Output` but with a prettier message than `.ok().unwrap()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::new("echo")
    ///     .args(&["42"])
    ///     .unwrap();
    /// ```
    fn unwrap(self) -> process::Output {
        match self.ok() {
            Ok(output) => output,
            Err(err) => panic!("{}", err),
        }
    }

    /// Unwrap a `std::process::Output` but with a prettier message than `.ok().unwrap()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::new("non_existent_command")
    ///     .args(&["42"])
    ///     .unwrap_err();
    /// ```
    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => panic!(
                "Command completed successfully\nstdout=```{}```",
                dump_buffer(&output.stdout)
            ),
            Err(err) => err,
        }
    }
}

impl OutputOkExt for process::Output {
    fn ok(self) -> OutputResult {
        if self.status.success() {
            Ok(self)
        } else {
            let error = OutputError::new(self);
            Err(error)
        }
    }
}

impl<'c> OutputOkExt for &'c mut process::Command {
    fn ok(self) -> OutputResult {
        let output = self.output().map_err(OutputError::with_cause)?;
        if output.status.success() {
            Ok(output)
        } else {
            let error = OutputError::new(output).set_cmd(format!("{:?}", self));
            Err(error)
        }
    }

    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => panic!(
                "Completed successfully:\ncommand=`{:?}`\nstdout=```{}```",
                self,
                dump_buffer(&output.stdout)
            ),
            Err(err) => err,
        }
    }
}

pub(crate) fn dump_buffer(buffer: &[u8]) -> String {
    if let Ok(buffer) = str::from_utf8(buffer) {
        format!("{}", buffer)
    } else {
        format!("{:?}", buffer)
    }
}
