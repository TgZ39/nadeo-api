use crate::auth::o_auth::OAuthInfo;
use std::rc::Rc;

use crate::auth::{AuthInfo, AuthType};
use crate::request::NadeoRequest;
use crate::{Error, Result};

use reqwest::{Client, IntoUrl, Method, Response};

use crate::client::client_builder::NadeoClientBuilder;
use crate::request::metadata::MetaData;
use crate::request::request_builder::NadeoRequestBuilder;
use thiserror::Error;

pub mod client_builder;

pub(crate) const NADEO_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
pub(crate) const NADEO_SERVER_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/basic";
pub(crate) const NADEO_REFRESH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/refresh";
pub(crate) const UBISOFT_APP_ID: &str = "86263886-327a-4328-ac69-527f0d20a237";
pub(crate) const EXPIRATION_TIME_BUFFER: i64 = 60;

/// This client can execute [`NadeoRequest`]s and handles authentication.
///
/// # Examples
///
/// Creating a client.
/// ```rust
/// # use nadeo_api::NadeoClient;
/// let mut client = NadeoClient::builder()
///     .with_normal_auth("email", "password") // optional (but at least 1 of the 3 is required)
///     .with_server_auth("username", "password") // optional
///     .with_oauth("identifier", "secret") // optional
///     .user_agent("Testing the API / mustermann.max@gmail.com") // required
///     .build()
///     .await?;
/// ```
///
/// [`NadeoRequest`]: NadeoRequest
#[derive(Debug, Clone)]
pub struct NadeoClient {
    pub(crate) client: Client,
    pub(crate) normal_auth: Option<Rc<AuthInfo>>,
    pub(crate) live_auth: Option<Rc<AuthInfo>>,
    pub(crate) o_auth: Option<Rc<OAuthInfo>>,
    pub(crate) meta_data: MetaData,
}

impl NadeoClient {
    pub fn builder() -> NadeoClientBuilder {
        NadeoClientBuilder::default()
    }

    /// Executes a [`NadeoRequest`] on the given [`NadeoClient`]. For more information about the API endpoints look [here](https://webservices.openplanet.dev/).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the required credentials for authorizing the request were not supplied when building the client or when there is an Error while executing the request.
    ///
    /// # Examples
    ///
    /// Gets the clubtag of a player given the *accountID*.
    /// ```rust
    /// # use nadeo_api::auth::AuthType;
    /// # use nadeo_api::NadeoClient;
    /// # use nadeo_api::request::{Method, NadeoRequest};
    ///
    /// // create client
    /// let mut client = NadeoClient::builder()
    ///     .with_normal_auth("email", "password")
    ///     .user_agent("Testing the API / mustermann.max@gmail.com")
    ///     .build()
    ///     .await?;
    ///
    /// // build request
    /// let request = NadeoRequest::builder()
    ///     .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578")
    ///     .auth_type(AuthType::NadeoServices)
    ///     .method(Method::GET)
    ///     .build()?;
    ///
    /// // execute request
    /// let response = client.execute(request).await?;
    /// ```
    ///
    /// [`Error`]: crate::Error
    /// [`NadeoRequest`]: NadeoRequest
    /// [`NadeoClient`]: NadeoClient
    pub async fn execute(&self, request: NadeoRequest) -> Result<Response> {
        match request.auth_type {
            AuthType::NadeoServices => {
                if let Some(auth) = &self.normal_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::from(ClientError::MissingCredentials {
                        auth_type: request.auth_type,
                    }))
                }
            }
            AuthType::NadeoLiveServices => {
                if let Some(auth) = &self.live_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::from(ClientError::MissingCredentials {
                        auth_type: request.auth_type,
                    }))
                }
            }
            AuthType::OAuth => {
                if let Some(auth) = &self.o_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::from(ClientError::MissingCredentials {
                        auth_type: request.auth_type,
                    }))
                }
            }
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::GET, url, auth_type)
    }

    pub fn post<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::POST, url, auth_type)
    }

    pub fn put<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::PUT, url, auth_type)
    }

    pub fn patch<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::PATCH, url, auth_type)
    }

    pub fn delete<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::DELETE, url, auth_type)
    }

    pub fn head<U: IntoUrl>(&self, url: U, auth_type: AuthType) -> Result<NadeoRequestBuilder> {
        self.request(Method::HEAD, url, auth_type)
    }

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

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("missing credentials to execute request")]
    MissingCredentials { auth_type: AuthType },
}
