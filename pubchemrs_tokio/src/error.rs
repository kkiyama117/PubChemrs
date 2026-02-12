//! Error types for the `pubchemrs_tokio` HTTP client crate.

use pubchemrs_struct::error::PubChemError;

/// Error type for `pubchemrs_tokio` operations, covering HTTP, API, and parsing failures.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error originating from `pubchemrs_struct` (invalid input, parse failure, etc.).
    #[error(transparent)]
    PubChem(#[from] PubChemError),

    /// An HTTP transport error from `reqwest`.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// A structured fault returned by the PubChem API (e.g., `PUGREST.BadRequest`).
    #[error("API fault: {code} - {message}")]
    ApiFault {
        /// The PubChem fault code (e.g., `"PUGREST.NotFound"`).
        code: String,
        /// Human-readable fault message.
        message: String,
    },

    /// A non-success HTTP status code with the response body.
    #[error("HTTP status {status}: {body}")]
    HttpStatus {
        /// The HTTP status code.
        status: u16,
        /// The response body text.
        body: String,
    },

    /// A JSON deserialization error.
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_pubchem() {
        let err = Error::PubChem(PubChemError::InvalidInput("bad input".into()));
        assert_eq!(err.to_string(), "Invalid Request: bad input");
    }

    #[test]
    fn test_error_display_api_fault() {
        let err = Error::ApiFault {
            code: "PUGREST.BadRequest".to_string(),
            message: "Invalid CID".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "API fault: PUGREST.BadRequest - Invalid CID"
        );
    }

    #[test]
    fn test_error_display_http_status() {
        let err = Error::HttpStatus {
            status: 404,
            body: "Not Found".to_string(),
        };
        assert_eq!(err.to_string(), "HTTP status 404: Not Found");
    }

    #[test]
    fn test_error_display_json() {
        let json_err = serde_json::from_str::<i32>("not_json").unwrap_err();
        let expected_msg = format!("JSON parse error: {json_err}");
        let err = Error::Json(json_err);
        assert_eq!(err.to_string(), expected_msg);
    }

    #[test]
    fn test_error_from_pubchem_error() {
        let pubchem_err = PubChemError::Unknown;
        let err: Error = pubchem_err.into();
        assert!(matches!(err, Error::PubChem(PubChemError::Unknown)));
    }

    #[test]
    fn test_error_from_serde_json_error() {
        let json_err = serde_json::from_str::<i32>("invalid").unwrap_err();
        let err: Error = json_err.into();
        assert!(matches!(err, Error::Json(_)));
    }

    #[test]
    fn test_error_is_debug() {
        let err = Error::HttpStatus {
            status: 500,
            body: "Internal Server Error".to_string(),
        };
        let debug = format!("{err:?}");
        assert!(debug.contains("HttpStatus"));
        assert!(debug.contains("500"));
    }
}
