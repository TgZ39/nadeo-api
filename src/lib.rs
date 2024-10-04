//! This crate provides an interface for working with the [Nadeo API](https://webservices.openplanet.dev/).
//! It handles (re)authentication automatically.
//!
//! # Getting started
//!
//! At first, you need to create a [`NadeoClient`] to execute [`NadeoRequest`]s.
//! You will need to provide credentials for at least one authentication method and a `UserAgent`.
//!
//! ```rust
//! # use nadeo_api::NadeoClient;
//!
//! let mut client = NadeoClient::builder()
//!     .with_normal_auth("ubisoft_account_email", "ubisoft_account_password")
//!     .with_server_auth("my_username", "my_password")
//!     .with_oauth("my_identifier", "my_secret")
//!     .user_agent("Testing the API / my.email@gmail.com")
//!     .build()
//!     .await?;
//! ```
//!
//! Use [`NadeoRequest::builder`] to create a `NadeoRequestBuilder`.
//! To create a [`NadeoRequest`] you will need to supply:
//! - an [`AuthType`]:
//!     - The depends on the API endpoint you want to make a request to.
//!       If the endpoint requires `AuthType::NadeoServices` or `AuthType::NadeoLiveServices` you need to build the [`NadeoClient`] with `NadeoClientBuilder::with_normal_auth()`.
//!       If the endpoint requires `AuthType::OAuth` you need to build the [`NadeoClient`] with `NadeoClientBuilder::with_oauth()`.
//! - an `URL`
//! - a [`Method`]
//!
//! For more information about the API endpoints look [here](https://webservices.openplanet.dev/).
//!
//! ```rust
//! # use nadeo_api::{NadeoClient, NadeoRequest};
//! # use nadeo_api::auth::AuthType;
//! # use nadeo_api::request::Method;
//!
//! let mut client = NadeoClient::builder()
//!     .with_normal_auth("ubisoft_account_email", "ubisoft_account_password")
//!     .with_oauth("my_identifier", "my_secret")
//!     .user_agent("Testing the API / my.email@gmail.com")
//!     .build()
//!     .await?;
//!
//! let request = NadeoRequest::builder()
//!     .auth_type(AuthType::NadeoServices)
//!     .url("some_url")
//!     .method(Method::GET)
//!     .build()?;
//! ```
//!
//! To execute the request use:
//!
//! ```rust
//! let res = client.execute(request).await?;
//! ```
//!
//! [`Method`]: request::Method
//! [`AuthType::NadeoServices`]: auth::AuthType::NadeoServices
//! [`AuthType::NadeoLiveServices`]: auth::AuthType::NadeoLiveServices
//! [`AuthType::OAuth`]: auth::AuthType::OAuth
//! [`AuthType`]: auth::AuthType
//! [`NadeoClientBuilder::with_normal_auth()`]: client::client_builder::NadeoClientBuilder::with_normal_auth
//! [`NadeoClientBuilder::with_oauth_auth()`]: client::client_builder::NadeoClientBuilder::with_oauth

pub mod auth;
pub mod client;
pub mod error;
pub mod request;

pub use error::{Error, Result};

pub use client::NadeoClient;
pub use request::NadeoRequest;
