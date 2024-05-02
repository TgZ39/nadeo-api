use crate::auth::token::access_token::AccessToken;
use crate::auth::token::refresh_token::RefreshToken;
use crate::auth::{AuthInfo, Service};
use crate::request::{HeaderMap, HttpMethod, NadeoRequest};
use crate::{auth, Error, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use futures::future::join;
use reqwest::{Client, Response};
use serde_json::{json, Value};
use std::str::FromStr;
use strum::Display;
use thiserror::Error;
use crate::auth::o_auth::OAuthInfo;

pub mod client_builder;

pub(crate) const NADEO_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
pub(crate) const NADEO_REFRESH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/refresh";
pub(crate) const UBISOFT_APP_ID: &str = "86263886-327a-4328-ac69-527f0d20a237";
pub(crate) const EXPIRATION_TIME_BUFFER: i64 = 60;

/// This client can execute [`NadeoRequest`]s and handles authentication. OAuth is not supported.
///
/// # Examples
///
/// Creating a client.
/// ```rust
/// # use nadeo_api::NadeoClient;
/// let mut client = NadeoClient::new("my_email", "my_password").await?;
/// ```
///
/// [`NadeoRequest`]: NadeoRequest
#[derive(Debug, Clone)]
pub struct NadeoClient {
    pub(crate) client: Client,
    pub(crate) normal_auth: Option<AuthInfo>,
    pub(crate) live_auth: Option<AuthInfo>,
    pub(crate) o_auth: Option<OAuthInfo>
}

impl NadeoClient {
    /// Creates a new [Client](NadeoClient) and gets the authtoken for *NadeoServices* and *NadeoLiveServices*.
    ///
    /// # Errors
    ///
    /// Returns an error if there was an error while authenticating with the Nadeo API. For example when the provided E-Mail or password is incorrect.
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        let client = Client::new();

        let ticket = auth::get_ubi_auth_ticket(email, password, &client).await?;
        let normal_auth_fut = AuthInfo::new(Service::NadeoServices, &ticket, &client);
        let live_auth_fut = AuthInfo::new(Service::NadeoLiveServices, &ticket, &client);

        // execute 2 futures concurrently
        let (normal_auth, live_auth) = join(normal_auth_fut, live_auth_fut).await;

        Ok(NadeoClient {
            client,
            normal_auth: None,
            live_auth: None,
            o_auth: None
        })
    }

    /// Executes a [`NadeoRequest`] on the given [`NadeoClient`]. For more information about the API endpoints look [here](https://webservices.openplanet.dev/).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`]
    ///
    /// # Examples
    ///
    /// Gets the clubtag of a player given the *accountID*.
    /// ```rust
    /// # use nadeo_api::auth::Service;
    /// # use nadeo_api::NadeoClient;
    /// # use nadeo_api::request::{HttpMethod, NadeoRequest};
    ///
    /// // create client
    /// let mut client = NadeoClient::new("my_email", "my_password").await?;
    ///
    /// // build request
    /// let request = NadeoRequest::builder()
    ///     .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578".to_string())
    ///     .service(Service::NadeoServices)
    ///     .http_method(HttpMethod::Get)
    ///     .build()?;
    ///
    /// // execute request
    /// let response = client.execute(request).await?;
    /// ```
    ///
    /// [`Error`]: crate::Error
    /// [`NadeoRequest`]: NadeoRequest
    /// [`NadeoClient`]: NadeoClient
    pub async fn execute(&mut self, request: NadeoRequest) -> Result<Response> {
        match request.service {
            Service::NadeoServices => {
                if let Some(auth) = &mut self.o_auth {
                    auth.refresh(&self.client).await?;
                } else {
                    return Err(Error::from(ClientError::MissingNormalAuth))
                }
            },
            Service::NadeoLiveServices => {
                if let Some(auth) = &mut self.live_auth {
                    auth.refresh(&self.client).await?;
                } else {
                    return Err(Error::from(ClientError::MissingNormalAuth))
                }
            },
            Service::OAuth => {
                if let Some(auth) = &mut self.o_auth {
                    auth.refresh(&self.client).await?;
                } else {
                    return Err(Error::from(ClientError::MissingOAuth))
                }
            }
        };

        let token = match request.service {
            Service::NadeoServices => {
                if let Some(token) = &self.normal_auth {
                    Some(token.access_token.clone().encode())
                } else {
                    None
                }
            },
            Service::NadeoLiveServices => {
                if let Some(token) = &self.live_auth {
                    Some(token.access_token.clone().encode())
                } else {
                    None
                }
            },
            Service::OAuth => {
                if let Some(token) = &self.o_auth {
                    Some(token.access_token.clone())
                } else {
                    None
                }
            }
        };
        if token.is_none() {
            return match request.service {
                Service::NadeoServices | Service::NadeoLiveServices => {
                    Err(Error::from(ClientError::MissingNormalAuth))
                },
                Service::OAuth => {
                    Err(Error::from(ClientError::MissingOAuth))
                }
            }
        }

        let auth_token = format!("nadeo_v1 t={}", token.unwrap());

        let mut headers = request.headers;
        headers.insert("Authorization", auth_token.parse().unwrap());

        let api_request = match request.method {
            HttpMethod::Get => self.client.get(request.url),
            HttpMethod::Post => self.client.post(request.url),
            HttpMethod::Put => self.client.put(request.url),
            HttpMethod::Patch => self.client.patch(request.url),
            HttpMethod::Delete => self.client.delete(request.url),
            HttpMethod::Head => self.client.head(request.url)
        };

        let res = api_request
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Client does not have credentials for NadeoServices or NadeoLiveServices")]
    MissingNormalAuth,
    #[error("Client does not have OAuth credentials")]
    MissingOAuth
}