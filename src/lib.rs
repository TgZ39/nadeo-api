//! This crate provides an interface for working with the [Nadeo API](https://webservices.openplanet.dev/).
//! It handles all authentication automatically.
//!
//! # Getting started
//!
//! The [`NadeoClient`] is thin wrapper around `reqwest::Client` and can execute [`NadeoRequest`]'s.
//! To create a `NadeoClient` we need to create a [`NadeoClientBuilder`] using [`NadeoClient::builder`].
//!
//! > Note that at least one of the 3 `NadeoClientBuilder::*_auth` needs to be provided.
//!
//! ```rust
//! # async {
//!  use nadeo_api::NadeoClient;
//!
//!  let client: NadeoClient = NadeoClient::builder()
//!     .user_agent("Foo Project / example@example.com") // required
//!     .with_normal_auth("ubisoft@example.com", "my_ubisoft_password")
//!     .with_server_auth("my_username", "my_server_password")
//!     .with_oauth("my_identifier", "my_secret")
//!     .build()
//!     .await?;
//! # }
//! ```
//!
//! To create a [`NadeoRequest`] we use [`NadeoClient::get`] or [`NadeoClient::post`] or any other http method.
//!
//! ```rust
//! # async {
//!  use nadeo_api::{NadeoClient, AuthType, Response, request::NadeoRequestBuilder};
//!  let url = "https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578";
//!  let req: NadeoRequestBuilder = client.get(url, AuthType::NadeoServices)?;
//!
//!  let resp: Response = req.send().await?;
//!  let body = resp.text().await?;
//!
//!  println!("Response: {body}");
//! # }
//! ```
//!
//! [`NadeoClient`]: client::NadeoClient
//! [`NadeoClient::builder`]: client::NadeoClient::builder
//! [`NadeoClientBuilder`]: client::client_builder::NadeoClientBuilder

pub mod auth;
pub mod client;
pub mod error;
pub mod request;

pub use crate::auth::AuthType;
pub use crate::client::NadeoClient;
pub use crate::error::{Error, Result};
pub use crate::request::NadeoRequest;

pub use reqwest::Method;
pub use reqwest::Response;
pub use reqwest::Url;
