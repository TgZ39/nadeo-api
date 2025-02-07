use crate::auth::oauth::OAuthInfo;
use crate::auth::{AuthInfo, AuthType};
use crate::error::*;
use crate::request::NadeoRequest;
use std::sync::Arc;

use reqwest::{Client, IntoUrl, Method, Response};

use crate::client::client_builder::NadeoClientBuilder;
use crate::request::metadata::MetaData;
use crate::request::request_builder::NadeoRequestBuilder;

pub mod client_builder;

pub(crate) const NADEO_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
pub(crate) const NADEO_SERVER_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/basic";
pub(crate) const NADEO_REFRESH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/refresh";
pub(crate) const UBISOFT_APP_ID: &str = "86263886-327a-4328-ac69-527f0d20a237";
pub(crate) const EXPIRATION_TIME_BUFFER: i64 = 60;

/// The [`NadeoClient`] is thin wrapper around `reqwest::Client` and can execute [`NadeoRequest`]'s.
/// To create a `NadeoClient` we need to create a [`NadeoClientBuilder`] using [`NadeoClient::builder`].
///
/// > Note that at least one of the 3 `NadeoClientBuilder::*_auth` needs to be provided.
///
/// ```rust
/// # async {
///  use nadeo_api::prelude::*;
///
///  let client: NadeoClient = NadeoClient::builder()
///     .user_agent("Foo Project / example@example.com") // required
///     .with_normal_auth("ubisoft@example.com", "my_ubisoft_password")
///     .with_server_auth("my_username", "my_server_password")
///     .with_oauth("my_identifier", "my_secret")
///     .build()
///     .await?;
/// # }
/// ```
///
/// To create a [`NadeoRequest`] we use [`NadeoClient::get`] or [`NadeoClient::post`] or any other http method.
///
/// ```rust
/// # async {
/// use nadeo_api::prelude::*;
///  let url = "https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578";
///  let req: NadeoRequestBuilder = client.get(url, AuthType::NadeoServices)?;
///
///  let resp: Response = req.send().await?;
///  let body = resp.text().await?;
///
///  println!("Response: {body}");
/// # }
/// ```
///
/// [`NadeoClient`]: client::NadeoClient
/// [`NadeoClient::builder`]: client::NadeoClient::builder
/// [`NadeoClientBuilder`]: client::client_builder::NadeoClientBuilder
#[derive(Debug, Clone)]
pub struct NadeoClient {
    pub(crate) client: Client,
    pub(crate) normal_auth: Option<Arc<AuthInfo>>,
    pub(crate) live_auth: Option<Arc<AuthInfo>>,
    pub(crate) o_auth: Option<Arc<OAuthInfo>>,
    pub(crate) meta_data: Arc<MetaData>,
}

impl NadeoClient {
    /// Creates a [`NadeoClientBuilder`].
    pub fn builder() -> NadeoClientBuilder {
        NadeoClientBuilder::default()
    }

    /// Executes a [`NadeoRequest`] on the given [`NadeoClient`].
    /// For more information about the API endpoints look [here](https://webservices.openplanet.dev/).
    ///
    /// # Errors
    ///
    /// Returns a [`Error::MissingCredentials`] error if the required authentication credentials were not provided in the [`NadeoClientBuilder`].
    ///
    /// # Examples
    ///
    /// Gets the clubtag of a player given the *accountID*.
    /// ```rust
    /// # async {
    /// use nadeo_api::prelude::*;
    ///
    ///  let client: NadeoClient = NadeoClient::builder()
    ///     .user_agent("Testing Project / example@example.com")
    ///     .with_normal_auth("my_email", "my_password")
    ///     .build()
    ///     .await?;
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
    pub async fn execute(&self, request: NadeoRequest) -> Result<Response> {
        match request.auth_type {
            AuthType::NadeoServices => {
                if let Some(auth) = &self.normal_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::MissingCredentials {
                        auth_type: request.auth_type,
                    })
                }
            }
            AuthType::NadeoLiveServices => {
                if let Some(auth) = &self.live_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::MissingCredentials {
                        auth_type: request.auth_type,
                    })
                }
            }
            AuthType::OAuth => {
                if let Some(auth) = &self.o_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::MissingCredentials {
                        auth_type: request.auth_type,
                    })
                }
            }
        }
    }

    /// Shorthand for `client.request(Method::GET, url, auth_type)`.
    pub fn get<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::GET, url, auth_type)
    }

    /// Shorthand for `client.request(Method::POST, url, auth_type)`.
    pub fn post<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::POST, url, auth_type)
    }

    /// Shorthand for `client.request(Method::PUT, url, auth_type)`.
    pub fn put<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::PUT, url, auth_type)
    }

    /// Shorthand for `client.request(Method::PATCH, url, auth_type)`.
    pub fn patch<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::PATCH, url, auth_type)
    }

    /// Shorthand for `client.request(Method::DELETE, url, auth_type)`.
    pub fn delete<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::DELETE, url, auth_type)
    }

    /// Shorthand for `client.request(Method::HEAD, url, auth_type)`.
    pub fn head<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::HEAD, url, auth_type)
    }

    /// Creates a [`NadeoRequestBuilder`].
    pub fn request<U: IntoUrl>(
        &self,
        method: Method,
        url: U,
        auth_type: AuthType,
    ) -> Result<NadeoRequestBuilder> {
        let req = NadeoRequest::new(method, url.into_url()?, auth_type);
        Ok(NadeoRequestBuilder::from_parts(self, req))
    }
}
