//! Simplify one-off runs of programs.

use std::error::Error;
use std::fmt;
use std::process;
use std::str;

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
/// [`std::process::Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
/// [`OutputResult`]: type.OutputResult.html
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
    /// [Output]: https://doc.rust-lang.org/std/process/struct.Output.html
    /// [OutputResult]: type.OutputResult.html
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
    /// [Output]: https://doc.rust-lang.org/std/process/struct.Output.html
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
    /// [Output]: https://doc.rust-lang.org/std/process/struct.Output.html
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
/// [`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`OutputOkExt`]: trait.OutputOkExt.html
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
/// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
/// [`OutputOkExt`]: trait.OutputOkExt.html
#[derive(Debug)]
pub struct OutputError {
    cmd: Option<String>,
    stdin: Option<Vec<u8>>,
    cause: OutputCause,
}

impl OutputError {
    /// Convert [`Output`] into an [`Error`].
    ///
    /// [`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
    /// [`Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
    pub fn new(output: process::Output) -> Self {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Expected(Output { output }),
        }
    }

    /// For errors that happen in creating a [`Output`].
    ///
    /// [`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
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
        self.stdin = Some(stdin);
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
    /// [`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
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
        if let Some(ref cmd) = self.cmd {
            writeln!(f, "command=`{}`", cmd)?;
        }
        if let Some(ref stdin) = self.stdin {
            if let Ok(stdin) = str::from_utf8(stdin) {
                writeln!(f, "stdin=```{}```", stdin)?;
            } else {
                writeln!(f, "stdin=```{:?}```", stdin)?;
            }
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
    if let Some(code) = output.status.code() {
        writeln!(f, "code={}", code)?;
    } else {
        writeln!(f, "code=<interrupted>")?;
    }

    write!(f, "stdout=```")?;
    write_buffer(&output.stdout, f)?;
    writeln!(f, "```")?;

    write!(f, "stderr=```")?;
    write_buffer(&output.stderr, f)?;
    writeln!(f, "```")?;

    Ok(())
}

pub(crate) fn dump_buffer(buffer: &[u8]) -> String {
    if let Ok(buffer) = str::from_utf8(buffer) {
        buffer.to_string()
    } else {
        format!("{:?}", buffer)
    }
}

pub(crate) fn write_buffer(buffer: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Ok(buffer) = str::from_utf8(buffer) {
        write!(f, "{}", buffer)
    } else {
        write!(f, "{:?}", buffer)
    }
}

#[derive(Debug)]
pub(crate) struct DebugBuffer {
    buffer: Vec<u8>,
}

impl DebugBuffer {
    pub(crate) fn new(buffer: Vec<u8>) -> Self {
        DebugBuffer { buffer }
    }
}

impl fmt::Display for DebugBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_buffer(&self.buffer, f)
    }
}
