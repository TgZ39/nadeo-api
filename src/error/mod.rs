use derive_more::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Display)]
pub enum Error {
    NadeoApi(#[from] reqwest::Error),
    Token(#[from] crate::auth::token::TokenError),
    Client(#[from] crate::client::client_builder::ClientCreationError),
    Request(#[from] crate::request::request_builder::RequestBuilderError),
}
