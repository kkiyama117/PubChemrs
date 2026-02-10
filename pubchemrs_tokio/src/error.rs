use pubchemrs_struct::error::PubChemError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PubChem(#[from] PubChemError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API fault: {code} - {message}")]
    ApiFault { code: String, message: String },

    #[error("HTTP status {status}: {body}")]
    HttpStatus { status: u16, body: String },

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
