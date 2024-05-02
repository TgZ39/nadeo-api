//! This crate provides an interface for working with the [Nadeo API](https://webservices.openplanet.dev/).
//! It handles (re)authentication automatically. OAuth is not supported *yet*.

pub mod auth;
pub mod client;
pub mod error;
pub mod request;

pub use error::{Error, Result};

pub use client::NadeoClient;
pub use request::NadeoRequest;
