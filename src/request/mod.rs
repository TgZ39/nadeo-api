use crate::auth::Service;
use crate::request::request_builder::NadeoRequestBuilder;

pub use reqwest::header::HeaderMap;
pub use reqwest::Response;
pub use reqwest::Url;

pub mod request_builder;

#[derive(Debug)]
pub struct NadeoRequest {
    pub(crate) service: Service,
    pub(crate) url: String,
    pub(crate) method: HttpMethod,
    pub(crate) headers: HeaderMap,
}

impl NadeoRequest {
    pub fn builder() -> NadeoRequestBuilder {
        NadeoRequestBuilder::default()
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
}
