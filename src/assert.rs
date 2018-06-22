use std::fmt;
use std::process;
use std::str;

use predicates;
use predicates::str::PredicateStrExt;

use cmd::dump_buffer;
use errors::output_fmt;

/// Assert the state of an `Output`.
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
///     .assert()
///     .success();
/// ```
pub trait OutputAssertExt {
    /// Wrap with an interface for that provides assertions on the `process::Output`.
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
    ///     .assert()
    ///     .success();
    /// ```
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

/// Assert the state of an `Output`.
///
/// Create an `Assert` through the `OutputAssertExt` trait.
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
///     .assert()
///     .success();
/// ```
#[derive(Debug)]
pub struct Assert {
    output: process::Output,
    cmd: Option<String>,
    stdin: Option<Vec<u8>>,
}

impl Assert {
    /// Create an `Assert` for a given `Output`.
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
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::main_binary()
    ///     .unwrap()
    ///     .assert()
    ///     .success();
    /// ```
    pub fn success(self) -> Self {
        if !self.output.status.success() {
            let actual_code = self.output.status.code().unwrap_or_else(|| {
                panic!(
                    "Unexpected failure.\ncode=<interrupted>\nstderr=```{}```\n{}",
                    dump_buffer(&self.output.stderr),
                    self
                )
            });
            panic!(
                "Unexpected failure.\ncode-{}\nstderr=```{}```\n{}",
                actual_code,
                dump_buffer(&self.output.stderr),
                self
            );
        }
        self
    }

    /// Ensure the command failed.
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
    ///     .env("exit", "1")
    ///     .assert()
    ///     .failure();
    /// ```
    pub fn failure(self) -> Self {
        if self.output.status.success() {
            panic!(
                "Unexpected success\nstdout=```{}```\n{}",
                dump_buffer(&self.output.stdout),
                self
            );
        }
        self
    }

    /// Ensure the command aborted before returning a code.
    pub fn interrupted(self) -> Self {
        if self.output.status.code().is_some() {
            panic!(
                "Unexpected completion\nstdout=```{}```\n{}",
                dump_buffer(&self.output.stdout),
                self
            );
        }
        self
    }

    /// Ensure the command returned the expected code.
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
    ///     .env("exit", "42")
    ///     .assert()
    ///     .code(42);
    /// ```
    pub fn code<I, P>(self, pred: I) -> Self
    where
        I: IntoCodePredicate<P>,
        P: predicates::Predicate<i32>,
    {
        self.code_impl(&pred.into_code())
    }

    fn code_impl(self, pred: &predicates::Predicate<i32>) -> Self {
        let actual_code = self.output.status.code().unwrap_or_else(|| {
            panic!(
                "Command interrupted\nstderr=```{}```\n{}",
                dump_buffer(&self.output.stderr),
                self
            )
        });
        if !pred.eval(&actual_code) {
            panic!(
                "Unexpected return code\nstdout=```{}```\nstderr=```{}```\n{}",
                dump_buffer(&self.output.stdout),
                dump_buffer(&self.output.stderr),
                self
            );
        }
        self
    }

    /// Ensure the command wrote the expected data to `stdout`.
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
    ///     .env("stdout", "hello")
    ///     .env("stderr", "world")
    ///     .assert()
    ///     .stdout("hello\n");
    /// ```
    pub fn stdout<I, P>(self, pred: I) -> Self
    where
        I: IntoOutputPredicate<P>,
        P: predicates::Predicate<[u8]>,
    {
        self.stdout_impl(&pred.into_output())
    }

    fn stdout_impl(self, pred: &predicates::Predicate<[u8]>) -> Self {
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
    /// ```rust
    /// use assert_cmd::prelude::*;
    ///
    /// use std::process::Command;
    ///
    /// Command::main_binary()
    ///     .unwrap()
    ///     .env("stdout", "hello")
    ///     .env("stderr", "world")
    ///     .assert()
    ///     .stderr("world\n");
    /// ```
    pub fn stderr<I, P>(self, pred: I) -> Self
    where
        I: IntoOutputPredicate<P>,
        P: predicates::Predicate<[u8]>,
    {
        self.stderr_impl(&pred.into_output())
    }

    fn stderr_impl(self, pred: &predicates::Predicate<[u8]>) -> Self {
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
            if let Ok(stdin) = str::from_utf8(stdin) {
                writeln!(f, "stdin=```{}```", stdin)?;
            } else {
                writeln!(f, "stdin=```{:?}```", stdin)?;
            }
        }
        output_fmt(&self.output, f)
    }
}

/// Used by `Assert::code` to convert `Self` into the needed `Predicate<i32>`.
///
/// # Examples
///
/// ```rust,ignore
/// use assert_cmd::prelude::*;
///
/// use std::process::Command;
///
/// Command::main_binary()
///     .unwrap()
///     .env("exit", "42")
///     .assert()
///     .code(42);
/// // which is equivalent to
/// Command::main_binary()
///     .unwrap()
///     .env("exit", "42")
///     .assert()
///     .code(predicates::ord::eq(42));
/// ```
pub trait IntoCodePredicate<P>
where
    P: predicates::Predicate<i32>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a program's exit code.
    fn into_code(self) -> P;
}

impl<P> IntoCodePredicate<P> for P
where
    P: predicates::Predicate<i32>,
{
    type Predicate = P;

    fn into_code(self) -> Self::Predicate {
        self
    }
}

impl IntoCodePredicate<predicates::ord::EqPredicate<i32>> for i32 {
    type Predicate = predicates::ord::EqPredicate<i32>;

    fn into_code(self) -> Self::Predicate {
        predicates::ord::eq(self)
    }
}

/// Used by `Assert` to convert Self into the needed `Predicate<[u8]>`.
pub trait IntoOutputPredicate<P>
where
    P: predicates::Predicate<[u8]>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a path.
    fn into_output(self) -> P;
}

impl<P> IntoOutputPredicate<P> for P
where
    P: predicates::Predicate<[u8]>,
{
    type Predicate = P;

    fn into_output(self) -> Self::Predicate {
        self
    }
}

impl IntoOutputPredicate<predicates::str::Utf8Predicate<predicates::ord::EqPredicate<&'static str>>>
    for &'static str
{
    type Predicate = predicates::str::Utf8Predicate<predicates::ord::EqPredicate<&'static str>>;

    fn into_output(self) -> Self::Predicate {
        predicates::ord::eq(self).from_utf8()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use predicates::prelude::*;

    // Since IntoCodePredicate exists solely for conversion, test it under that scenario to ensure
    // it works as expected.
    fn convert_code<I, P>(pred: I) -> P
    where
        I: IntoCodePredicate<P>,
        P: predicates::Predicate<i32>,
    {
        pred.into_code()
    }

    #[test]
    fn into_code_from_pred() {
        let pred = convert_code(predicate::eq(10));
        assert!(pred.eval(&10));
    }

    #[test]
    fn into_code_from_i32() {
        let pred = convert_code(10);
        assert!(pred.eval(&10));
    }

    // Since IntoOutputPredicate exists solely for conversion, test it under that scenario to ensure
    // it works as expected.
    fn convert_output<I, P>(pred: I) -> P
    where
        I: IntoOutputPredicate<P>,
        P: predicates::Predicate<[u8]>,
    {
        pred.into_output()
    }

    #[test]
    fn into_output_from_pred() {
        let pred = convert_output(predicate::eq(b"Hello" as &[u8]));
        assert!(pred.eval(b"Hello" as &[u8]));
    }

    #[test]
    fn into_output_from_str() {
        let pred = convert_output("Hello");
        assert!(pred.eval(b"Hello" as &[u8]));
    }
}
