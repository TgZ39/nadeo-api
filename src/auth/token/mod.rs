use thiserror::Error;

pub mod access_token;
pub mod refresh_token;

/// Errors for deserializing encoded tokens.
#[derive(Error, Debug)]
pub enum ParseTokenError {
    #[error("encoded token is invalid")]
    InvalidInput,
    #[error("base64 encoding of the payload is invalid: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("payload could not be deserialized: {0}")]
    Json(#[from] serde_json::Error),
}
