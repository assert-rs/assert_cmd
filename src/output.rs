//! Simplify one-off runs of programs.

use bstr::ByteSlice;
use std::error::Error;
use std::fmt;
use std::process;

/// Converts a type to an [`OutputResult`].
///
/// This is for example implemented on [`std::process::Output`].
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let result = Command::new("echo")
///     .args(&["42"])
///     .ok();
/// assert!(result.is_ok());
/// ```
///
pub trait OutputOkExt
where
    Self: ::std::marker::Sized,
{
    /// Convert an [`Output`][Output] to an [`OutputResult`][OutputResult].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let result = Command::new("echo")
    ///     .args(&["42"])
    ///     .ok();
    /// assert!(result.is_ok());
    /// ```
    ///
    /// [Output]: std::process::Output
    fn ok(self) -> OutputResult;

    /// Unwrap a [`Output`][Output] but with a prettier message than `.ok().unwrap()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let output = Command::new("echo")
    ///     .args(&["42"])
    ///     .unwrap();
    /// ```
    ///
    /// [Output]: std::process::Output
    fn unwrap(self) -> process::Output {
        match self.ok() {
            Ok(output) => output,
            Err(err) => panic!("{}", err),
        }
    }

    /// Unwrap a [`Output`][Output] but with a prettier message than `ok().err().unwrap()`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let err = Command::new("a-command")
    ///     .args(&["--will-fail"])
    ///     .unwrap_err();
    /// ```
    ///
    /// [Output]: std::process::Output
    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => panic!(
                "Command completed successfully\nstdout=```{}```",
                DebugBytes::new(&output.stdout)
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
                DebugBytes::new(&output.stdout)
            ),
            Err(err) => err,
        }
    }
}

/// [`Output`] represented as a [`Result`].
///
/// Generally produced by [`OutputOkExt`].
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let result = Command::new("echo")
///     .args(&["42"])
///     .ok();
/// assert!(result.is_ok());
/// ```
///
/// [`Output`]: std::process::Output
/// [`Result`]: std::result::Result
pub type OutputResult = Result<process::Output, OutputError>;

/// [`Command`] error.
///
/// Generally produced by [`OutputOkExt`].
///
/// # Examples
///
/// ```rust,no_run
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let err = Command::new("a-command")
///     .args(&["--will-fail"])
///     .unwrap_err();
/// ```
///
/// [`Command`]: std::process::Command
#[derive(Debug)]
pub struct OutputError {
    cmd: Option<String>,
    stdin: Option<bstr::BString>,
    cause: OutputCause,
}

impl OutputError {
    /// Convert [`Output`] into an [`Error`].
    ///
    /// [`Output`]: std::process::Output
    /// [`Error`]: std::error::Error
    pub fn new(output: process::Output) -> Self {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Expected(Output { output }),
        }
    }

    /// For errors that happen in creating a [`Output`].
    ///
    /// [`Output`]: std::process::Output
    pub fn with_cause<E>(cause: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Unexpected(Box::new(cause)),
        }
    }

    /// Add the command line for additional context.
    pub fn set_cmd(mut self, cmd: String) -> Self {
        self.cmd = Some(cmd);
        self
    }

    /// Add the `stdin` for additional context.
    pub fn set_stdin(mut self, stdin: Vec<u8>) -> Self {
        self.stdin = Some(bstr::BString::from(stdin));
        self
    }

    /// Access the contained [`Output`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let err = Command::new("a-command")
    ///     .args(&["--will-fail"])
    ///     .unwrap_err();
    /// let output = err
    ///     .as_output()
    ///     .unwrap();
    /// assert_eq!(Some(42), output.status.code());
    /// ```
    ///
    /// [`Output`]: std::process::Output
    pub fn as_output(&self) -> Option<&process::Output> {
        match self.cause {
            OutputCause::Expected(ref e) => Some(&e.output),
            OutputCause::Unexpected(_) => None,
        }
    }
}

impl Error for OutputError {}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = crate::Palette::current();
        if let Some(ref cmd) = self.cmd {
            writeln!(
                f,
                "{}={}",
                palette.key.paint("command"),
                palette.value.paint(cmd)
            )?;
        }
        if let Some(ref stdin) = self.stdin {
            writeln!(
                f,
                "{}={}",
                palette.key.paint("stdin"),
                palette.value.paint(DebugBytes::new(stdin))
            )?;
        }
        write!(f, "{}", self.cause)
    }
}

#[derive(Debug)]
enum OutputCause {
    Expected(Output),
    Unexpected(Box<dyn Error + Send + Sync + 'static>),
}

impl fmt::Display for OutputCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OutputCause::Expected(ref e) => write!(f, "{}", e),
            OutputCause::Unexpected(ref e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
struct Output {
    output: process::Output,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        output_fmt(&self.output, f)
    }
}

pub(crate) fn output_fmt(output: &process::Output, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let palette = crate::Palette::current();
    if let Some(code) = output.status.code() {
        writeln!(
            f,
            "{}={}",
            palette.key.paint("code"),
            palette.value.paint(code)
        )?;
    } else {
        writeln!(
            f,
            "{}={}",
            palette.key.paint("code"),
            palette.value.paint("<interrupted>")
        )?;
    }

    write!(
        f,
        "{}={}\n{}={}\n",
        palette.key.paint("stdout"),
        palette.value.paint(DebugBytes::new(&output.stdout)),
        palette.key.paint("stderr"),
        palette.value.paint(DebugBytes::new(&output.stderr)),
    )?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct DebugBytes<'a> {
    bytes: &'a [u8],
}

impl<'a> DebugBytes<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        DebugBytes { bytes }
    }
}

impl<'a> fmt::Display for DebugBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_bytes(self.bytes, f)
    }
}

#[derive(Debug)]
pub(crate) struct DebugBuffer {
    buffer: bstr::BString,
}

impl DebugBuffer {
    pub(crate) fn new(buffer: Vec<u8>) -> Self {
        DebugBuffer {
            buffer: buffer.into(),
        }
    }
}

impl fmt::Display for DebugBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_bytes(&self.buffer, f)
    }
}

fn format_bytes(data: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #![allow(clippy::assertions_on_constants)]
    const MIN_OVERFLOW: usize = 8192;
    const MAX_START: usize = 2048;
    const MAX_END: usize = 2048;
    const MAX_PRINTED: usize = MAX_START + MAX_END;
    assert!(MAX_PRINTED < MIN_OVERFLOW);

    if data.len() >= MIN_OVERFLOW {
        write!(
            f,
            "<{} bytes total>{:?}...<{} bytes omitted>...{:?}",
            data.len(),
            data[..MAX_START].as_bstr(),
            data.len() - MAX_PRINTED,
            data[data.len() - MAX_END..].as_bstr(),
        )
    } else {
        write!(f, "{:?}", data.as_bstr())
    }
}
