use std::fmt;
use std::io::Write;
use std::io;
use std::process;
use std::str;

use failure;
use predicates;

/// Extend `Command` with a helper to pass a buffer to `stdin`
pub trait CommandStdInExt {
    /// Write `buffer` to `stdin` when the command is run.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
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

/// `std::process::Command` with a `stdin` buffer.
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

/// `std::process::Output` represented as a `Result`.
pub type OutputResult = Result<process::Output, OutputError>;

/// Extends `std::process::Output` with methods to to convert it to an `OutputResult`.
pub trait OutputOkExt
where
    Self: ::std::marker::Sized,
{
    /// Convert an `std::process::Output` into an `OutputResult`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
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
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
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
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
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

fn output_fmt(output: &process::Output, f: &mut fmt::Formatter) -> fmt::Result {
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
            if let Ok(stdin) = str::from_utf8(&stdin) {
                writeln!(f, "stdin=```{}```", stdin)?;
            } else {
                writeln!(f, "stdin=```{:?}```", stdin)?;
            }
        }
        write!(f, "{}", self.cause)
    }
}

/// Extend `process::Output` with assertions.
///
/// # Examples
///
/// ```rust,ignore
/// extern crate assert_cli;
/// use std::process::Command;
/// use assert_cli::cmd::*;
///
/// Command::main_binary()
///     .assert()
///     .success();
/// ```
pub trait OutputAssertExt {
    /// Wrap with an interface for that provides assertions on the `process::Output`.
    fn assert(self) -> Assert;
}

impl OutputAssertExt for process::Output {
    fn assert(self) -> Assert {
        Assert::new(self)
    }
}

impl<'c> OutputAssertExt for &'c mut process::Command {
    fn assert(self) -> Assert {
        let output = self.output().unwrap();
        Assert::new(output).set_cmd(format!("{:?}", self))
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

/// `process::Output` assertions.
#[derive(Debug)]
pub struct Assert {
    output: process::Output,
    cmd: Option<String>,
    stdin: Option<Vec<u8>>,
}

impl Assert {
    /// Convert `std::process::Output` into a `Fail`.
    pub fn new(output: process::Output) -> Self {
        Self {
            output,
            cmd: None,
            stdin: None,
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
    pub fn get_output(&self) -> &process::Output {
        &self.output
    }

    // How does user interact with assertion API?
    // - On Assert class, using error chaining
    //   - "Builder" or not?  If yes, then do we extend Result?
    //   - How do we give a helpful unwrap?
    // - Build up assertion data and "execute" it, like assert_cli used to?  But that was mostly
    //   from building up before executing the command happened.  Now we're doing it
    //   after-the-fact.
    // - Immediately panic in each assertion? Let's give that a try.

    /// Ensure the command succeeded.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
    ///
    /// Command::main_binary()
    ///     .assert()
    ///     .success();
    /// ```
    pub fn success(self) -> Self {
        if !self.output.status.success() {
            panic!("Unexpected failure\n{}", self);
        }
        self
    }

    /// Ensure the command failed.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
    ///
    /// Command::main_binary()
    ///     .env("exit", 1)
    ///     .assert()
    ///     .failure();
    /// ```
    pub fn failure(self) -> Self {
        if self.output.status.success() {
            panic!("Unexpected success\n{}", self);
        }
        self
    }

    /// Ensure the command returned the expected code.
    pub fn interrupted(self) -> Self {
        if self.output.status.code().is_some() {
            panic!("Unexpected completion\n{}", self);
        }
        self
    }

    /// Ensure the command returned the expected code.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
    ///
    /// Command::main_binary()
    ///     .env("exit", "42")
    ///     .assert()
    ///     .code(predicates::ord::eq(42));
    /// ```
    pub fn code(self, pred: &predicates::Predicate<i32>) -> Self {
        let actual_code = self.output
            .status
            .code()
            .unwrap_or_else(|| panic!("Command interrupted\n{}", self));
        if !pred.eval(&actual_code) {
            panic!("Unexpected return code\n{}", self);
        }
        self
    }

    /// Ensure the command wrote the expected data to `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
    ///
    /// Command::main_binary()
    ///     .env("stdout", "hello")
    ///     .env("stderr", "world")
    ///     .assert()
    ///     .stdout(predicates::ord::eq(b"hello"));
    /// ```
    pub fn stdout(self, pred: &predicates::Predicate<Vec<u8>>) -> Self {
        {
            let actual = &self.output.stdout;
            if !pred.eval(actual) {
                panic!("Unexpected stdout\n{}", self);
            }
        }
        self
    }

    /// Ensure the command wrote the expected data to `stderr`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// extern crate assert_cli;
    /// use std::process::Command;
    /// use assert_cli::cmd::*;
    ///
    /// Command::main_binary()
    ///     .env("stdout", "hello")
    ///     .env("stderr", "world")
    ///     .assert()
    ///     .stderr(predicates::ord::eq(b"world"));
    /// ```
    pub fn stderr(self, pred: &predicates::Predicate<Vec<u8>>) -> Self {
        {
            let actual = &self.output.stderr;
            if !pred.eval(actual) {
                panic!("Unexpected stderr\n{}", self);
            }
        }
        self
    }
}

impl fmt::Display for Assert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref cmd) = self.cmd {
            writeln!(f, "command=`{}`", cmd)?;
        }
        if let Some(ref stdin) = self.stdin {
            if let Ok(stdin) = str::from_utf8(&stdin) {
                writeln!(f, "stdin=```{}```", stdin)?;
            } else {
                writeln!(f, "stdin=```{:?}```", stdin)?;
            }
        }
        output_fmt(&self.output, f)
    }
}

fn dump_buffer(buffer: &[u8]) -> String {
    if let Ok(buffer) = str::from_utf8(&buffer) {
        format!("{}", buffer)
    } else {
        format!("{:?}", buffer)
    }
}
