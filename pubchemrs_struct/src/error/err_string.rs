use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;

use super::ERROR_STRATEGY;
use super::ErrorStrategy;

/// A lightweight, cloneable error message string.
///
/// `ErrString` wraps a `Cow<'static, str>` and respects the global error
/// strategy set via environment variables (`PUBCHEM_PANIC_ON_ERR`,
/// `PUBCHEM_BACKTRACE_IN_ERR`). When created via `From<T>`, the message
/// may be enriched with a backtrace or trigger a panic depending on the
/// active strategy.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "pyo3", pyo3::pyclass)]
pub struct ErrString(Cow<'static, str>);

impl ErrString {
    /// Create an `ErrString` from a static string literal at compile time.
    ///
    /// This bypasses the global error strategy and never panics or captures a backtrace.
    pub const fn new_static(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl<T> From<T> for ErrString
where
    T: Into<Cow<'static, str>>,
{
    fn from(msg: T) -> Self {
        match &*ERROR_STRATEGY {
            ErrorStrategy::Panic => panic!("{}", msg.into()),
            ErrorStrategy::WithBacktrace => ErrString(Cow::Owned(format!(
                "{}\n\nRust backtrace:\n{}",
                msg.into(),
                std::backtrace::Backtrace::force_capture()
            ))),
            ErrorStrategy::Normal => ErrString(msg.into()),
        }
    }
}

impl AsRef<str> for ErrString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ErrString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ErrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
