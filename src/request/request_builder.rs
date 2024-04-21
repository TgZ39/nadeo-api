use crate::auth::Service;
use crate::request::{HttpMethod, NadeoRequest};
use reqwest::header::{HeaderMap, IntoHeaderName};
use thiserror::Error;

pub struct NadeoRequestBuilder {
    service: Option<Service>,
    url: Option<String>,
    method: Option<HttpMethod>,
    headers: HeaderMap,
}

macro_rules! builder_fn {
    ( $builder_struct:ty, $field:ident, $fn_name:ident, $val:ty ) => {
        impl $builder_struct {
            pub fn $fn_name(mut self, val: $val) -> Self {
                self.$field = Some(val);
                self
            }
        }
    };
}

builder_fn!(NadeoRequestBuilder, url, url, String);
builder_fn!(NadeoRequestBuilder, method, http_method, HttpMethod);
builder_fn!(NadeoRequestBuilder, service, service, Service);

impl Default for NadeoRequestBuilder {
    fn default() -> Self {
        NadeoRequestBuilder {
            service: None,
            method: None,
            headers: HeaderMap::new(),
            url: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum RequestBuilderError {
    #[error("No URL was provided")]
    MissingUrl,
    #[error("No HTTP-method was provided")]
    MissingHttpMethod,
    #[error("No NadeoServiceType was provided")]
    MissingService,
}

impl NadeoRequestBuilder {
    pub fn add_header<K>(&mut self, key: K, val: String) -> &mut Self
    where
        K: IntoHeaderName,
    {
        self.headers.insert(key, val.parse().unwrap());
        self
    }

    pub fn build(self) -> Result<NadeoRequest, RequestBuilderError> {
        if self.url.is_none() {
            return Err(RequestBuilderError::MissingUrl);
        }
        if self.method.is_none() {
            return Err(RequestBuilderError::MissingHttpMethod);
        }
        if self.service.is_none() {
            return Err(RequestBuilderError::MissingService);
        }

        Ok(NadeoRequest {
            service: self.service.unwrap(),
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            headers: self.headers,
        })
    }
}
