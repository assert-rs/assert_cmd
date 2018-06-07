use std::io;
use std::io::Write;
use std::process;

use assert::Assert;
use assert::OutputAssertExt;
use cmd::OutputOkExt;
use cmd::dump_buffer;
use errors::OutputError;
use errors::OutputResult;

/// Write to `stdin` of a `Command`.
pub trait CommandStdInExt {
    /// Write `buffer` to `stdin` when the command is run.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::new("cat")
    ///     .with_stdin("42")
    ///     .unwrap();
    /// ```
    fn with_stdin<S>(self, buffer: S) -> StdInCommand
    where
        S: Into<Vec<u8>>;
}

impl CommandStdInExt for process::Command {
    fn with_stdin<S>(self, buffer: S) -> StdInCommand
    where
        S: Into<Vec<u8>>,
    {
        StdInCommand {
            cmd: self,
            stdin: buffer.into(),
        }
    }
}

/// `Command` that carries the `stdin` buffer.
///
/// Create a `StdInCommand` through the `CommandStdInExt` trait.
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// Command::new("cat")
///     .with_stdin("42")
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct StdInCommand {
    cmd: process::Command,
    stdin: Vec<u8>,
}

impl StdInCommand {
    /// Executes the command as a child process, waiting for it to finish and collecting all of its
    /// output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the resulting output).
    /// Stdin is not inherited from the parent and any attempt by the child process to read from
    /// the stdin stream will result in the stream immediately closing.
    ///
    /// *(mirrors `std::process::Command::output`**
    pub fn output(&mut self) -> io::Result<process::Output> {
        self.spawn()?.wait_with_output()
    }

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// *(mirrors `std::process::Command::spawn`**
    fn spawn(&mut self) -> io::Result<process::Child> {
        // stdout/stderr should only be piped for `output` according to `process::Command::new`.
        self.cmd.stdin(process::Stdio::piped());
        self.cmd.stdout(process::Stdio::piped());
        self.cmd.stderr(process::Stdio::piped());

        let mut spawned = self.cmd.spawn()?;

        spawned
            .stdin
            .as_mut()
            .expect("Couldn't get mut ref to command stdin")
            .write_all(&self.stdin)?;
        Ok(spawned)
    }
}

impl<'c> OutputOkExt for &'c mut StdInCommand {
    fn ok(self) -> OutputResult {
        let output = self.output().map_err(OutputError::with_cause)?;
        if output.status.success() {
            Ok(output)
        } else {
            let error = OutputError::new(output)
                .set_cmd(format!("{:?}", self.cmd))
                .set_stdin(self.stdin.clone());
            Err(error)
        }
    }

    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => panic!(
                "Completed successfully:\ncommand=`{:?}`\nstdin=```{}```\nstdout=```{}```",
                self.cmd,
                dump_buffer(&self.stdin),
                dump_buffer(&output.stdout)
            ),
            Err(err) => err,
        }
    }
}

impl<'c> OutputAssertExt for &'c mut StdInCommand {
    fn assert(self) -> Assert {
        let output = self.output().unwrap();
        Assert::new(output)
            .set_cmd(format!("{:?}", self.cmd))
            .set_stdin(self.stdin.clone())
    }
}
