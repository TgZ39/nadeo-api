use crate::auth::AuthType;
use reqwest::header::HeaderMap;
use std::time::Duration;

pub use reqwest::Method;
pub use reqwest::Response;
use reqwest::{Body, Request, Url, Version};

pub mod request_builder;

pub(crate) mod metadata;

/// Contains information about an API request. NadeoRequests can be executed on an instance of a [`NadeoClient`].
/// If you want to create a request use the [`NadeoRequestBuilder`] with `NadeoRequest::builder()`.
///
/// # Examples
///
/// Gets the clubtag of a player given the *accountID*.
/// ```rust
/// # use reqwest::Method;
/// # use nadeo_api::auth::AuthType;
/// # use nadeo_api::request::NadeoRequest;
///
/// let mut client = //snap;
///
/// let request = NadeoRequest::builder()
///     .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578")
///     .auth_type(AuthType::NadeoServices)
///     .method(Method::GET)
///     .build()?;
///
/// let response = client.execute(request).await?;
/// ```
///
/// [`NadeoClient`]: crate::client::NadeoClient
/// [`NadeoRequestBuilder`]: NadeoRequestBuilder

/// A request which can be executed using `NadeoClient::execute`.
///
/// `NadeoRequest` is a thin wrapper around [`reqwest::Request`] that tries to mimic its functionality as closely as possible.
#[derive(Debug)]
pub struct NadeoRequest {
    pub(crate) request: Request,
    pub(crate) auth_type: AuthType,
}

impl NadeoRequest {
    pub fn new(method: Method, url: Url, auth_type: AuthType) -> Self {
        Self {
            request: Request::new(method, url),
            auth_type,
        }
    }

    pub fn method(&self) -> &Method {
        self.request.method()
    }

    pub fn method_mut(&mut self) -> &mut Method {
        self.request.method_mut()
    }

    pub fn url(&self) -> &Url {
        self.request.url()
    }

    pub fn url_mut(&mut self) -> &mut Url {
        self.request.url_mut()
    }

    pub fn headers(&self) -> &HeaderMap {
        self.request.headers()
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.request.headers_mut()
    }

    pub fn body(&self) -> Option<&Body> {
        self.request.body()
    }

    pub fn body_mut(&mut self) -> &mut Option<Body> {
        self.request.body_mut()
    }

    pub fn timeout(&self) -> Option<&Duration> {
        self.request.timeout()
    }

    pub fn timeout_mut(&mut self) -> &mut Option<Duration> {
        self.request.timeout_mut()
    }

    pub fn version(&self) -> Version {
        self.request.version()
    }

    pub fn version_mut(&mut self) -> &mut Version {
        self.request.version_mut()
    }

    pub fn auth_type(&self) -> &AuthType {
        &self.auth_type
    }

    pub fn auth_type_mut(&mut self) -> &mut AuthType {
        &mut self.auth_type
    }

    pub fn try_clone(&self) -> Option<Self> {
        self.request.try_clone().map(|request| Self {
            request,
            auth_type: self.auth_type,
        })
    }
}
