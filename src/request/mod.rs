use crate::auth::AuthType;
use crate::request::request_builder::NadeoRequestBuilder;
use reqwest::header::HeaderMap;

pub use reqwest::Method;
pub use reqwest::Response;

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

#[derive(Debug, Clone)]
pub struct NadeoRequest {
    pub(crate) auth_type: AuthType,
    pub(crate) url: String,
    pub(crate) method: Method,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Option<String>,
}

impl NadeoRequest {
    /// Creates a new [`NadeoRequestBuilder`]. This is the only way of creating a [NadeoRequest].
    ///
    /// [`NadeoRequest`]: NadeoRequest
    pub fn builder() -> NadeoRequestBuilder {
        NadeoRequestBuilder::default()
    }
}
