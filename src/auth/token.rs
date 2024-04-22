use derive_more::Display;
use thiserror::Error;

pub use access_token::AccessToken;
pub use refresh_token::RefreshToken;

pub mod access_token;
pub mod refresh_token;

#[derive(Error, Display, Debug)]
pub enum TokenError {
    InvalidInput,
    Base64(#[from] base64::DecodeError),
    Json(#[from] serde_json::Error),
}
