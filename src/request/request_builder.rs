use crate::auth::Service;
use crate::request::{HttpMethod, NadeoRequest};
use crate::{Error, Result};
use reqwest::header::{HeaderMap, IntoHeaderName};
use serde::{Deserialize, Serialize};

/// Used for creating [`NadeoRequest`]s. <br>
/// The `URL`, [`HttpMethod`] and [`Service`] must be provided to successfully *build* a request.
///
/// [`NadeoRequest`]: NadeoRequest
/// [`HttpMethod`]: HttpMethod
/// [`Service`]: Service
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

/// Error when the Request is invalid. For example if a required field is missing.
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum RequestBuilderError {
    #[error("no URL was provided")]
    MissingUrl,
    #[error("no HTTP method was provided")]
    MissingHttpMethod,
    #[error("no NadeoService was provided")]
    MissingService,
}

impl NadeoRequestBuilder {
    /// Adds a header to the request. Adding a header should not be required in most cases.
    ///
    /// # Panics
    ///
    /// Panics if there is an error parsing the value.
    pub fn add_header<K>(mut self, key: K, val: &str) -> Self
    where
        K: IntoHeaderName,
    {
        self.headers.insert(key, val.parse().unwrap());
        self
    }

    /// Converts the `NadeoRequestBuilder` into a [`NadeoRequest`].
    /// The `URL`, [`HttpMethod`] and [`Service`] are required for a request.
    ///
    /// [`NadeoRequest`]: NadeoRequest
    /// [`HttpMethod`]: HttpMethod
    /// [`Service`]: Service
    pub fn build(self) -> Result<NadeoRequest> {
        if self.url.is_none() {
            return Err(Error::from(RequestBuilderError::MissingUrl));
        }
        if self.method.is_none() {
            return Err(Error::from(RequestBuilderError::MissingHttpMethod));
        }
        if self.service.is_none() {
            return Err(Error::from(RequestBuilderError::MissingService));
        }

        Ok(NadeoRequest {
            service: self.service.unwrap(),
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            headers: self.headers,
        })
    }
}
