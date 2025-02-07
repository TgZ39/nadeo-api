use crate::auth::AuthType;
use crate::client::NadeoClient;
use crate::error::*;
use crate::request::NadeoRequest;
use http::{HeaderMap, HeaderName, HeaderValue, Version};
use reqwest::multipart::Form;
use reqwest::{Body, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A builder to construct the properties of a [`NadeoRequest`].
/// To construct a `NadeoRequestBuilder`, refer to the [`NadeoClient`] documentation.
pub struct NadeoRequestBuilder {
    pub(crate) client: NadeoClient,
    pub(crate) request: RequestBuilder,
    pub(crate) auth_type: AuthType,
}

/// Error when the Request is invalid. For example if a required field is missing.
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum NadeoRequestBuilderError {
    #[error("no URL was provided")]
    MissingUrl,
    #[error("no HTTP method was provided")]
    MissingHttpMethod,
    #[error("no AuthType was provided")]
    MissingAuthType,
}

impl NadeoRequestBuilder {
    /// Assemble a builder starting from an existing [`NadeoClient`] and a [`NadeoRequest`].
    pub fn from_parts(client: &NadeoClient, request: NadeoRequest) -> Self {
        Self {
            client: client.clone(),
            request: RequestBuilder::from_parts(client.client.clone(), request.request),
            auth_type: request.auth_type,
        }
    }

    /// Add a `Header` to this Request.
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

    /// Add a set of Headers to the existing ones on this Request.
    /// The headers will be merged in to any already set.
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.request = self.request.headers(headers);

        self
    }

    /// Set the request body.
    pub fn body<T: Into<Body>>(mut self, body: T) -> Self {
        self.request = self.request.body(body);

        self
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the response body has finished.
    /// It affects only this request and overrides the timeout configured using [`NadeoClientBuilder::timeout`].
    ///
    /// [`NadeoClientBuilder::timeout`]: crate::client::client_builder::NadeoClientBuilder::timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.request = self.request.timeout(timeout);

        self
    }

    /// See https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.multipart
    pub fn multipart(mut self, multipart: Form) -> Self {
        self.request = self.request.multipart(multipart);

        self
    }

    /// See https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.query
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request = self.request.query(query);

        self
    }

    /// Set HTTP version.
    pub fn version(mut self, version: Version) -> Self {
        self.request = self.request.version(version);

        self
    }

    /// See https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.form
    pub fn form<T: Serialize + ?Sized>(mut self, form: &T) -> Self {
        self.request = self.request.form(form);

        self
    }

    /// See https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.json
    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.request = self.request.json(json);

        self
    }

    /// Disable CORS on fetching the request.
    pub fn fetch_mode_no_cors(mut self) -> Self {
        self.request = self.request.fetch_mode_no_cors();

        self
    }

    /// Build a [`NadeoRequest`], which can be inspected, modified and executed with [`NadeoClient::execute`].
    pub fn build(self) -> Result<NadeoRequest> {
        let request = self.request.build()?;

        Ok(NadeoRequest {
            request,
            auth_type: self.auth_type,
        })
    }

    /// Build a [`NadeoRequest`], which can be inspected, modified and executed with [`NadeoClient::execute`].
    /// This is similar to [`NadeoRequestBuilder::build`], but also returns the embedded Client.
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
    /// Constructs the Request and sends it to the target URL, returning a future Response.
    ///
    /// # Errors
    /// This method fails if there was an error while sending request, redirect loop was detected or redirect limit was exhausted.
    ///
    /// # Example
    /// ```rust
    /// # async {
    ///  use nadeo_api::{NadeoClient, AuthType, Response, request::NadeoRequestBuilder};
    ///
    ///  let url = "https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578";
    ///  let req: NadeoRequestBuilder = client.get(url, AuthType::NadeoServices)?;
    ///
    ///  let resp: Response = req.send().await?;
    ///  let body = resp.text().await?;
    ///
    ///  println!("Response: {body}");
    /// # }
    /// ```
    pub async fn send(self) -> Result<Response> {
        self.client.clone().execute(self.build()?).await
    }

    /// Attempt to clone the RequestBuilder.
    /// `None` is returned if the RequestBuilder can not be cloned, i.e. if the request body is a stream.
    pub fn try_clone(&self) -> Option<Self> {
        self.request.try_clone().map(|request| Self {
            client: self.client.clone(),
            request,
            auth_type: self.auth_type,
        })
    }
}
