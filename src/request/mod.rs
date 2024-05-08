use crate::auth::AuthType;
use crate::request::request_builder::NadeoRequestBuilder;
use reqwest::header::HeaderMap;

pub use reqwest::Response;
use serde::{Deserialize, Serialize};

pub mod request_builder;

pub(crate) mod metadata;

/// Contains information about an API request. NadeoRequests can be executed on an instance of a [`NadeoClient`].
/// If you want to create a request use the [`NadeoRequestBuilder`] with `NadeoRequest::builder()`.
///
/// # Examples
///
/// Gets the clubtag of a player given the *accountID*.
/// ```rust
/// # use nadeo_api::auth::AuthType;
/// # use nadeo_api::request::{HttpMethod, NadeoRequest};
///
/// let mut client = //snap;
///
/// let request = NadeoRequest::builder()
///          .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578")
///          .auth_type(AuthType::NadeoServices)
///          .http_method(HttpMethod::Get)
///          .build()?;
///
/// let response = client.execute(request).await?;
/// ```
///
/// [`NadeoClient`]: crate::client::NadeoClient
/// [`NadeoRequestBuilder`]: NadeoRequestBuilder

#[derive(Debug, Clone)]
pub struct NadeoRequest {
    pub(crate) auth_type: AuthType,
    pub(crate) url: String,
    pub(crate) method: HttpMethod,
    pub(crate) headers: HeaderMap,
}

impl NadeoRequest {
    /// Creates a new NadeoRequestBuilder. This is the only way of creating a [NadeoRequest].
    ///
    /// [`NadeoRequest`]: NadeoRequest
    pub fn builder() -> NadeoRequestBuilder {
        NadeoRequestBuilder::default()
    }
}

/// The HTTP method used for the API request.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}
