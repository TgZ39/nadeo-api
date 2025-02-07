use crate::auth::AuthType;
use http::Method;
pub use request_builder::{NadeoRequestBuilder, NadeoRequestBuilderError};
use reqwest::header::HeaderMap;
use reqwest::{Body, Request, Url, Version};
use std::time::Duration;

pub mod request_builder;

pub(crate) mod metadata;

/// The [`NadeoRequest`] is a thin wrapper around `reqwest::Request`.
/// A `NadeoRequest` can be executed on a [`NadeoClient`].
///
/// [`NadeoClient`]: crate::client::NadeoClient
#[derive(Debug)]
pub struct NadeoRequest {
    pub(crate) request: Request,
    pub(crate) auth_type: AuthType,
}

impl NadeoRequest {
    /// Constructs a new request.
    pub fn new(method: Method, url: Url, auth_type: AuthType) -> Self {
        Self {
            request: Request::new(method, url),
            auth_type,
        }
    }

    /// Get the method.
    pub fn method(&self) -> &Method {
        self.request.method()
    }

    /// Get a mutable reference to the method.
    pub fn method_mut(&mut self) -> &mut Method {
        self.request.method_mut()
    }

    /// Get the url.
    pub fn url(&self) -> &Url {
        self.request.url()
    }

    /// Get a mutable reference to the url.
    pub fn url_mut(&mut self) -> &mut Url {
        self.request.url_mut()
    }

    /// Get the headers.
    pub fn headers(&self) -> &HeaderMap {
        self.request.headers()
    }

    /// Get a mutable reference to the headers.
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.request.headers_mut()
    }

    /// Get the body.
    pub fn body(&self) -> Option<&Body> {
        self.request.body()
    }

    /// Get a mutable reference to the body.
    pub fn body_mut(&mut self) -> &mut Option<Body> {
        self.request.body_mut()
    }

    /// Get the timeout.
    pub fn timeout(&self) -> Option<&Duration> {
        self.request.timeout()
    }

    /// Get a mutable reference to the timeout.
    pub fn timeout_mut(&mut self) -> &mut Option<Duration> {
        self.request.timeout_mut()
    }

    /// Get the http version.
    pub fn version(&self) -> Version {
        self.request.version()
    }

    /// Get a mutable reference to the http version.
    pub fn version_mut(&mut self) -> &mut Version {
        self.request.version_mut()
    }

    /// Get the [`AuthType`].
    pub fn auth_type(&self) -> &AuthType {
        &self.auth_type
    }

    /// Get a mutable reference to the [`AuthType`].
    pub fn auth_type_mut(&mut self) -> &mut AuthType {
        &mut self.auth_type
    }

    /// Attempt to clone the request.
    /// `None` is returned if the request can not be cloned, i.e. if the body is a stream.
    pub fn try_clone(&self) -> Option<Self> {
        self.request.try_clone().map(|request| Self {
            request,
            auth_type: self.auth_type,
        })
    }
}
