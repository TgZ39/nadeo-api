use crate::auth::AuthType;
use crate::request::NadeoRequest;
use crate::{Error, NadeoClient, Result};
use http::{HeaderMap, HeaderName, HeaderValue, Version};
use reqwest::multipart::Form;
use reqwest::{Body, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Used for creating [`NadeoRequest`]s.
/// `URL`, [`Method`] and [`AuthType`] must be provided.
///
/// [`NadeoRequest`]: NadeoRequest
/// [`Method`]: Method
/// [`AuthType`]: AuthType
pub struct NadeoRequestBuilder {
    pub(crate) client: NadeoClient,
    pub(crate) request: RequestBuilder,
    pub(crate) auth_type: AuthType,
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
    pub fn from_parts(client: &NadeoClient, request: NadeoRequest) -> Self {
        Self {
            client: client.clone(),
            request: RequestBuilder::from_parts(client.client.clone(), request.request),
            auth_type: request.auth_type,
        }
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.request = self.request.header(key, value);

        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.request = self.request.headers(headers);

        self
    }

    pub fn body<T: Into<Body>>(mut self, body: T) -> Self {
        self.request = self.request.body(body);

        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.request = self.request.timeout(timeout);

        self
    }

    pub fn multipart(mut self, multipart: Form) -> Self {
        self.request = self.request.multipart(multipart);

        self
    }

    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request = self.request.query(query);

        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.request = self.request.version(version);

        self
    }

    pub fn form<T: Serialize + ?Sized>(mut self, form: &T) -> Self {
        self.request = self.request.form(form);

        self
    }

    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.request = self.request.json(json);

        self
    }

    pub fn fetch_mode_no_cors(mut self) -> Self {
        self.request = self.request.fetch_mode_no_cors();

        self
    }

    pub fn build(self) -> Result<NadeoRequest> {
        let request = self.request.build()?;

        Ok(NadeoRequest {
            request,
            auth_type: self.auth_type,
        })
    }

    pub fn build_split(self) -> (NadeoClient, Result<NadeoRequest>) {
        let request = match self.request.build() {
            Ok(req) => req,
            Err(err) => return (self.client, Err(Error::from(err))),
        };

        let nadeo_request = NadeoRequest {
            request,
            auth_type: self.auth_type,
        };
        (self.client, Ok(nadeo_request))
    }

    pub async fn send(self) -> Result<Response> {
        self.client.clone().execute(self.build()?).await
    }

    pub fn try_clone(&self) -> Option<Self> {
        self.request.try_clone().map(|request| Self {
            client: self.client.clone(),
            request,
            auth_type: self.auth_type,
        })
    }
}
