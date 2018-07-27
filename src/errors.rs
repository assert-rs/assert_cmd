use std::error::Error;
use std::fmt;
use std::process;
use std::str;

/// `std::process::Output` represented as a `Result`.
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
pub type OutputResult = Result<process::Output, OutputError>;

/// `Command` error.
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let err = Command::main_binary()
///     .unwrap()
///     .env("exit", "42")
///     .unwrap_err();
/// ```
#[derive(Debug)]
pub struct OutputError {
    cmd: Option<String>,
    stdin: Option<Vec<u8>>,
    cause: OutputCause,
}

impl OutputError {
    /// Convert `std::process::Output` into an `Error`.
    pub fn new(output: process::Output) -> Self {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Expected(Output { output }),
        }
    }

    /// For errors that happen in creating a `std::process::Output`.
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

    /// Access the contained `std::process::Output`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let err = Command::main_binary()
    ///     .unwrap()
    ///     .env("exit", "42")
    ///     .unwrap_err();
    /// let output = err
    ///     .as_output()
    ///     .unwrap();
    /// assert_eq!(Some(42), output.status.code());
    /// ```
    pub fn as_output(&self) -> Option<&process::Output> {
        match self.cause {
            OutputCause::Expected(ref e) => Some(&e.output),
            OutputCause::Unexpected(_) => None,
        }
    }
}

impl Error for OutputError {
    fn description(&self) -> &str {
        "Command failed."
    }

    fn cause(&self) -> Option<&Error> {
        if let OutputCause::Unexpected(ref err) = self.cause {
            Some(err.as_ref())
        } else {
            None
        }
    }
}
impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    Unexpected(Box<Error + Send + Sync + 'static>),
}

impl fmt::Display for OutputCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OutputCause::Expected(ref e) => write!(f, "{}", e),
            OutputCause::Unexpected(ref e) => write!(f, "{}", e),
        }
    }
}

/// Wrap `Output` to be `Dislay`able.
#[derive(Debug)]
struct Output {
    output: process::Output,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        output_fmt(&self.output, f)
    }
}

pub(crate) fn output_fmt(output: &process::Output, f: &mut fmt::Formatter) -> fmt::Result {
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

pub(crate) fn write_buffer(buffer: &[u8], f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_buffer(&self.buffer, f)
    }
}
