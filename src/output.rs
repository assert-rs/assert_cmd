//! Simplify one-off runs of programs.

use bstr::ByteSlice;
use std::error::Error;
use std::fmt::{self, Write};
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

fn format_bytes(data: &[u8], f: &mut impl fmt::Write) -> fmt::Result {
    #![allow(clippy::assertions_on_constants)]

    const LINES_MIN_OVERFLOW: usize = 40;
    const LINES_MAX_START: usize = 10;
    const LINES_MAX_END: usize = 10;
    const LINES_MAX_PRINTED: usize = LINES_MAX_START + LINES_MAX_END;
    assert!(LINES_MAX_PRINTED < LINES_MIN_OVERFLOW);

    const BYTES_MIN_OVERFLOW: usize = 8192;
    const BYTES_MAX_START: usize = 2048;
    const BYTES_MAX_END: usize = 2048;
    const BYTES_MAX_PRINTED: usize = BYTES_MAX_START + BYTES_MAX_END;
    assert!(BYTES_MAX_PRINTED < BYTES_MIN_OVERFLOW);

    let data_as_bstr = restore_newlines(&format!("{:?}", data.as_bstr()))?;

    // Strip quotes at beginning and end.
    let lines = data_as_bstr[1..data_as_bstr.len() - 1]
        .lines()
        .collect::<Vec<_>>();

    if lines.len() >= LINES_MIN_OVERFLOW {
        write!(
            f,
            "<{} lines total>\"{}\"\n<{} lines omitted>\n\"{}\"",
            lines.len(),
            lines[..LINES_MAX_START].join("\n"),
            lines.len() - LINES_MAX_PRINTED,
            lines[lines.len() - LINES_MAX_END..].join("\n"),
        )
    } else if data.len() >= BYTES_MIN_OVERFLOW {
        write!(
            &mut NewlineRestorer::new(f),
            "<{} bytes total>{:?}...<{} bytes omitted>...{:?}",
            data.len(),
            data[..BYTES_MAX_START].as_bstr(),
            data.len() - BYTES_MAX_PRINTED,
            data[data.len() - BYTES_MAX_END..].as_bstr(),
        )
    } else {
        f.write_str(&data_as_bstr)
    }
}

fn restore_newlines(s: &str) -> Result<String, fmt::Error> {
    let mut buf = String::new();
    NewlineRestorer::new(&mut buf).write_str(s)?;
    Ok(buf)
}

struct NewlineRestorer<'a, T>
where
    T: fmt::Write,
{
    inner: &'a mut T,
    trailing_backslash: bool,
}

impl<'a, T> NewlineRestorer<'a, T>
where
    T: fmt::Write,
{
    fn new(inner: &'a mut T) -> Self {
        Self {
            inner,
            trailing_backslash: false,
        }
    }
}

impl<'a, T> fmt::Write for NewlineRestorer<'a, T>
where
    T: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buf = if self.trailing_backslash {
            String::from("\\")
        } else {
            String::new()
        };

        buf += s;

        let trailing_backslashes = buf.chars().rev().take_while(|&c| c == '\\').count();

        self.trailing_backslash = if trailing_backslashes % 2 != 0 {
            buf.pop();
            true
        } else {
            false
        };

        self.inner.write_str(&buf.replace("\\n", "\n"))
    }
}

impl<'a, T> Drop for NewlineRestorer<'a, T>
where
    T: fmt::Write,
{
    fn drop(&mut self) {
        if self.trailing_backslash {
            self.inner.write_char('\\').unwrap_or_default();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn format_bytes() {
        let mut s = String::new();
        for i in 0..40 {
            s.push_str(&format!("{}\n", i));
        }
        let mut buf = String::new();
        super::format_bytes(s.as_bytes(), &mut buf).unwrap();
        assert_eq!(
            r#"<40 lines total>"0
1
2
3
4
5
6
7
8
9"
<20 lines omitted>
"30
31
32
33
34
35
36
37
38
39""#,
            buf
        );
    }

    #[test]
    fn restore_newlines() {
        let s = r#"escaped nul\0unescaped newline
escaped newline\n<end>"#;
        assert_eq!(
            r#"escaped nul\0unescaped newline
escaped newline
<end>"#,
            super::restore_newlines(s).unwrap()
        );
    }

    #[test]
    fn restore_newlines_trailing_backslashes() {
        let mut s = String::from("trailing backslashes");
        for _ in 0..4 {
            s.push('\\');
            assert_eq!(s, super::restore_newlines(&s).unwrap());
        }
    }
}
