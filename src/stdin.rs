//! Write to `stdin` of a [`Command`][Command].
//!
//! [Command]: https://doc.rust-lang.org/std/process/struct.Command.html

use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path;
use std::process;

use crate::assert::Assert;
use crate::assert::OutputAssertExt;
use crate::cmd::dump_buffer;
use crate::cmd::DebugBuffer;
use crate::cmd::OutputError;
use crate::cmd::OutputOkExt;
use crate::cmd::OutputResult;

/// Write to `stdin` of a [`Command`][Command].
///
/// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
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
    /// let mut cmd = Command::new("cat");
    /// cmd
    ///     .arg("-et");
    /// cmd
    ///     .with_stdin()
    ///     .buffer("42")
    ///     .assert()
    ///     .stdout("42");
    /// ```
    fn with_stdin(&mut self) -> StdInCommandBuilder<'_>;
}

impl CommandStdInExt for process::Command {
    fn with_stdin(&mut self) -> StdInCommandBuilder<'_> {
        StdInCommandBuilder { cmd: self }
    }
}

/// For adding a stdin to a [`Command`][Command].
///
/// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
#[derive(Debug)]
pub struct StdInCommandBuilder<'a> {
    cmd: &'a mut process::Command,
}

impl<'a> StdInCommandBuilder<'a> {
    /// Write `buffer` to `stdin` when the [`Command`][Command] is run.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::new("cat");
    /// cmd
    ///     .arg("-et");
    /// cmd
    ///     .with_stdin()
    ///     .buffer("42")
    ///     .assert()
    ///     .stdout("42");
    /// ```
    ///
    /// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
    pub fn buffer<S>(&mut self, buffer: S) -> StdInCommand<'_>
    where
        S: Into<Vec<u8>>,
    {
        StdInCommand {
            cmd: self.cmd,
            stdin: buffer.into(),
        }
    }

    /// Write `path`s content to `stdin` when the [`Command`][Command] is run.
    ///
    /// Paths are relative to the [`env::current_dir`][env_current_dir] and not
    /// [`Command::current_dir`][Command_current_dir].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// let mut cmd = Command::new("cat");
    /// cmd
    ///     .arg("-et");
    /// cmd
    ///     .with_stdin()
    ///     .buffer("42")
    ///     .assert()
    ///     .stdout("42");
    /// ```
    ///
    /// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [env_current_dir]: https://doc.rust-lang.org/std/env/fn.current_dir.html
    /// [Command_current_dir]: https://doc.rust-lang.org/std/process/struct.Command.html#method.current_dir
    pub fn path<P>(&mut self, file: P) -> io::Result<StdInCommand<'_>>
    where
        P: AsRef<path::Path>,
    {
        let file = file.as_ref();
        let mut buffer = Vec::new();
        fs::File::open(file)?.read_to_end(&mut buffer)?;
        Ok(StdInCommand {
            cmd: self.cmd,
            stdin: buffer,
        })
    }
}

/// [`Command`][Command] that carries the `stdin` buffer.
///
/// Create a `StdInCommand` through the [`CommandStdInExt`][CommandStdInExt] trait.
///
/// # Examples
///
/// ```rust
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// let mut cmd = Command::new("cat");
/// cmd
///     .arg("-et");
/// cmd
///     .with_stdin()
///     .buffer("42")
///     .assert()
///     .stdout("42");
/// ```
///
/// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
/// [CommandStdInExt]: trait.CommandStdInExt.html
#[derive(Debug)]
pub struct StdInCommand<'a> {
    cmd: &'a mut process::Command,
    stdin: Vec<u8>,
}

impl<'a> StdInCommand<'a> {
    /// Executes the [`Command`][Command] as a child process, waiting for it to finish and collecting all of its
    /// output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the resulting output).
    /// Stdin is not inherited from the parent and any attempt by the child process to read from
    /// the stdin stream will result in the stream immediately closing.
    ///
    /// *(mirrors [`Command::output`][Command_output])*
    ///
    /// [Command]: https://doc.rust-lang.org/std/process/struct.Command.html
    /// [Command_output]: https://doc.rust-lang.org/std/process/struct.Command.html#method.output
    pub fn output(&mut self) -> io::Result<process::Output> {
        self.spawn()?.wait_with_output()
    }

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

impl<'c, 'a> OutputOkExt for &'c mut StdInCommand<'a> {
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

impl<'c> OutputAssertExt for &'c mut StdInCommand<'c> {
    fn assert(self) -> Assert {
        let output = self.output().unwrap();
        Assert::new(output)
            .append_context("command", format!("{:?}", self.cmd))
            .append_context("stdin", DebugBuffer::new(self.stdin.clone()))
    }
}
