use derive_more::Display;

pub type Result<T> = core::result::Result<T, Error>;

/// Error type used across the crate.
#[derive(thiserror::Error, Debug, Display)]
pub enum Error {
    /// Errors for failed Nadeo API requests.
    NadeoApi(#[from] reqwest::Error),
    /// Errors for deserializing tokens.
    Token(#[from] crate::auth::token::ParseTokenError),
    /// Errors for invalid [`NadeoRequest`].
    ///
    /// [`NadeoRequest`]: crate::NadeoRequest
    Request(#[from] crate::request::request_builder::RequestBuilderError),
}
