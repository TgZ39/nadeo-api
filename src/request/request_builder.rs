use crate::auth::AuthType;
use crate::request::{HttpMethod, NadeoRequest};
use crate::{Error, Result};
use reqwest::header::{HeaderMap, IntoHeaderName};
use serde::{Deserialize, Serialize};

/// Used for creating [`NadeoRequest`]s. <br>
/// The `URL`, [`HttpMethod`] and [`AuthType`] must be provided to successfully *build* a request.
///
/// [`NadeoRequest`]: NadeoRequest
/// [`HttpMethod`]: HttpMethod
/// [`AuthType`]: AuthType
#[derive(Default)]
pub struct NadeoRequestBuilder {
    auth_type: Option<AuthType>,
    url: Option<String>,
    method: Option<HttpMethod>,
    headers: HeaderMap,
    body: Option<String>,
}

/// Error when the Request is invalid. For example if a required field is missing.
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum RequestBuilderError {
    #[error("no URL was provided")]
    MissingUrl,
    #[error("no HTTP method was provided")]
    MissingHttpMethod,
    #[error("no AuthType was provided")]
    MissingAuthType,
}

impl NadeoRequestBuilder {
    /// Adds a text body to the request. Usually JSON.
    pub fn body(mut self, json: &str) -> Self {
        self.body = Some(json.to_string());

        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());

        self
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);

        self
    }

    pub fn auth_type(mut self, auth_type: AuthType) -> Self {
        self.auth_type = Some(auth_type);

        self
    }

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
    /// The `URL`, [`HttpMethod`] and [`AuthType`] are required for a request.
    ///
    /// [`NadeoRequest`]: NadeoRequest
    /// [`HttpMethod`]: HttpMethod
    /// [`AuthType`]: AuthType
    pub fn build(self) -> Result<NadeoRequest> {
        if self.url.is_none() {
            return Err(Error::from(RequestBuilderError::MissingUrl));
        }
        if self.method.is_none() {
            return Err(Error::from(RequestBuilderError::MissingHttpMethod));
        }
        if self.auth_type.is_none() {
            return Err(Error::from(RequestBuilderError::MissingAuthType));
        }

        Ok(NadeoRequest {
            auth_type: self.auth_type.unwrap(),
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            headers: self.headers,
            body: self.body,
        })
    }
}
