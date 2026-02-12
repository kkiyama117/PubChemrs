//! Error types for the `pubchemrs_struct` crate.
//!
//! This module defines [`PubChemError`], the primary error enum, along with
//! helper types such as [`ErrString`] and [`ParseEnumError`]. Error behavior
//! can be controlled via environment variables:
//!
//! - `PUBCHEM_PANIC_ON_ERR=1` — panic instead of returning errors.
//! - `PUBCHEM_BACKTRACE_IN_ERR=1` — capture a backtrace in error messages.

mod err_string;

pub use err_string::ErrString;
use std::env;
use std::sync::LazyLock;
use thiserror::Error;

enum ErrorStrategy {
    Panic,
    WithBacktrace,
    Normal,
}

static ERROR_STRATEGY: LazyLock<ErrorStrategy> = LazyLock::new(|| {
    if env::var("PUBCHEM_PANIC_ON_ERR").as_deref() == Ok("1") {
        ErrorStrategy::Panic
    } else if env::var("PUBCHEM_BACKTRACE_IN_ERR").as_deref() == Ok("1") {
        ErrorStrategy::WithBacktrace
    } else {
        ErrorStrategy::Normal
    }
});

/// A type alias for `Result<T, PubChemError>`.
pub type PubChemResult<T> = std::result::Result<T, PubChemError>;

/// Error returned when parsing a string into an enum variant fails.
///
/// This replaces `strum::ParseError` to avoid a dependency on the `strum` crate.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseEnumError {
    /// No enum variant matched the input string.
    #[error("Matching variant not found")]
    VariantNotFound,
}

/// The primary error type for `pubchemrs_struct` operations.
#[derive(Debug, Error)]
pub enum PubChemError {
    /// The request input was invalid (e.g., empty identifiers, unsupported namespace).
    #[error("Invalid Request: {0}")]
    InvalidInput(ErrString),

    /// Failed to parse a PubChem API response.
    #[error("Parse Error: {0}")]
    ParseResponseError(ErrString),

    /// An enum variant could not be parsed from a string.
    #[error(transparent)]
    ParseEnum(#[from] ParseEnumError),

    /// An unknown or unclassified error occurred.
    #[error("Unknown Error")]
    Unknown,
}

impl From<std::convert::Infallible> for PubChemError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_enum_error_display() {
        let err = ParseEnumError::VariantNotFound;
        assert_eq!(err.to_string(), "Matching variant not found");
    }

    #[test]
    fn test_pubchem_error_from_parse_enum() {
        let err: PubChemError = ParseEnumError::VariantNotFound.into();
        assert!(matches!(
            err,
            PubChemError::ParseEnum(ParseEnumError::VariantNotFound)
        ));
    }

    #[test]
    fn test_pubchem_error_invalid_input() {
        let err = PubChemError::InvalidInput("bad input".into());
        assert_eq!(err.to_string(), "Invalid Request: bad input");
    }

    #[test]
    fn test_pubchem_error_parse_response() {
        let err = PubChemError::ParseResponseError("bad json".into());
        assert_eq!(err.to_string(), "Parse Error: bad json");
    }

    #[test]
    fn test_pubchem_error_unknown() {
        let err = PubChemError::Unknown;
        assert_eq!(err.to_string(), "Unknown Error");
    }

    #[test]
    fn test_err_string_from_str() {
        let es = ErrString::from("hello");
        assert_eq!(es.to_string(), "hello");
    }

    #[test]
    fn test_err_string_from_string() {
        let es = ErrString::from(String::from("world"));
        assert_eq!(es.to_string(), "world");
    }
}
