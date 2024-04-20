use crate::api::auth::auth_info::Service;
use crate::api::nadeo_request::presets::core::CoreRequest;
use crate::api::nadeo_request::presets::live::LiveRequest;
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

pub enum RequestPreset {
    Core(CoreRequest),
    Live(LiveRequest),
}

pub trait ToRequest {
    fn to_request(&self) -> NadeoRequest;
}

pub(crate) fn as_comma_list(list: &Vec<String>) -> String {
    let mut out = String::new();

    for entry in list {
        out.push_str(entry);
        out.push(',')
    }
    out.pop(); // remove trailing comma

    out
}
