use derive_more::Display;

pub type Result<T> = core::result::Result<T, Error>;

/// Error type used across the crate.
#[derive(thiserror::Error, Debug, Display)]
pub enum Error {
    NadeoApi(#[from] reqwest::Error),
    Client(#[from] crate::client::ClientError),
    ClientBuilderError(#[from] crate::client::client_builder::NadeoClientBuilderError),
    Token(#[from] crate::auth::token::ParseTokenError),
    Request(#[from] crate::request::request_builder::RequestBuilderError),
}
