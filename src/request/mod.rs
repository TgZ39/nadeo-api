use crate::auth::Service;
use crate::request::request_builder::NadeoRequestBuilder;

pub use reqwest::header::HeaderMap;
pub use reqwest::Response;
pub use reqwest::Url;

pub mod request_builder;

/// Contains information about an API request. NadeoRequests can be executed on an instance of a [NadeoClient](crate::client::NadeoClient).
/// If you want to create a request use the [NadeoRequestBuilder](NadeoRequestBuilder) with `NadeoRequest::builder()`.
///
/// # Examples
///
/// ```rust
/// # use nadeo_api::auth::Service;
/// # use nadeo_api::request::{HttpMethod, NadeoRequest};
///
/// let mut client = //snap;
///
/// let request = NadeoRequest::builder()
///          .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578".to_string())
///          .service(Service::NadeoServices)
///          .http_method(HttpMethod::Get)
///          .build()?;
///
/// let response = client.execute(request).await?;
/// ```
#[derive(Debug)]
pub struct NadeoRequest {
    pub(crate) service: Service,
    pub(crate) url: String,
    pub(crate) method: HttpMethod,
    pub(crate) headers: HeaderMap,
}

impl NadeoRequest {
    /// Creates a new NadeoRequestBuilder. This is the only way of creating a [NadeoRequest](NadeoRequest).
    pub fn builder() -> NadeoRequestBuilder {
        NadeoRequestBuilder::default()
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
}
