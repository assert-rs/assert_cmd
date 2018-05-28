use std::fmt;
use std::process;
use std::str;

use failure;

/// `std::process::Output` represented as a `Result`.
pub type OutputResult = Result<process::Output, OutputError>;

/// `std::process::Output` as a `Fail`.
#[derive(Fail, Debug)]
pub struct OutputError {
    cmd: Option<String>,
    stdin: Option<Vec<u8>>,
    cause: OutputCause,
}

impl OutputError {
    /// Convert `std::process::Output` into a `Fail`.
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
        E: Into<failure::Error>,
    {
        Self {
            cmd: None,
            stdin: None,
            cause: OutputCause::Unexpected(cause.into()),
        }
    }

    /// Add the command line for additional context.
    pub fn set_cmd(mut self, cmd: String) -> Self {
        self.cmd = Some(cmd);
        self
    }

    /// Add the `stdn` for additional context.
    pub fn set_stdin(mut self, stdin: Vec<u8>) -> Self {
        self.stdin = Some(stdin);
        self
    }

    /// Access the contained `std::process::Output`.
    pub fn as_output(&self) -> Option<&process::Output> {
        match self.cause {
            OutputCause::Expected(ref e) => Some(&e.output),
            OutputCause::Unexpected(_) => None,
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
    Unexpected(failure::Error),
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
#[derive(Fail, Debug)]
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
    if let Ok(stdout) = str::from_utf8(&output.stdout) {
        writeln!(f, "stdout=```{}```", stdout)?;
    } else {
        writeln!(f, "stdout=```{:?}```", output.stdout)?;
    }
    if let Ok(stderr) = str::from_utf8(&output.stderr) {
        writeln!(f, "stderr=```{}```", stderr)?;
    } else {
        writeln!(f, "stderr=```{:?}```", output.stderr)?;
    }

    Ok(())
}
