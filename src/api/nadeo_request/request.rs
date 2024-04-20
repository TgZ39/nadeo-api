use crate::api::auth::auth_info::Service;
use crate::api::nadeo_request::request_builder::{HttpMethod, NadeoRequestBuilder};
use reqwest::header::HeaderMap;

#[derive(Debug)]
pub struct NadeoRequest {
    pub service: Service,
    pub url: String,
    pub method: HttpMethod,
    pub headers: HeaderMap,
}

impl NadeoRequest {
    pub fn builder() -> NadeoRequestBuilder {
        NadeoRequestBuilder::default()
    }
}
