use crate::auth::o_auth::OAuthInfo;

use crate::auth::{AuthInfo, Service};
use crate::request::{HttpMethod, NadeoRequest};
use crate::{Error, Result};

use reqwest::{Client, Response};

use crate::client::client_builder::NadeoClientBuilder;
use thiserror::Error;

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
    pub(crate) o_auth: Option<OAuthInfo>,
}

impl NadeoClient {
    pub fn builder() -> NadeoClientBuilder {
        NadeoClientBuilder::default()
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
        // refresh tokens if required
        match request.service {
            Service::NadeoServices => {
                if let Some(auth) = &mut self.o_auth {
                    auth.refresh(&self.client).await?;
                } else {
                    return Err(Error::from(ClientError::MissingNormalAuth));
                }
            }
            Service::NadeoLiveServices => {
                if let Some(auth) = &mut self.live_auth {
                    auth.refresh(&self.client).await?;
                } else {
                    return Err(Error::from(ClientError::MissingNormalAuth));
                }
            }
            Service::OAuth => {
                if let Some(auth) = &mut self.o_auth {
                    auth.refresh(&self.client).await?;
                } else {
                    return Err(Error::from(ClientError::MissingOAuth));
                }
            }
        };

        // get token
        let token = match request.service {
            Service::NadeoServices => self
                .normal_auth
                .as_ref()
                .map(|token| format!("nadeo_v1 t={}", token.access_token.encode())),
            Service::NadeoLiveServices => self
                .live_auth
                .as_ref()
                .map(|token| format!("nadeo_v1 t={}", token.access_token.encode())),
            Service::OAuth => self
                .o_auth
                .as_ref()
                .map(|token| format!("Bearer {}", token.access_token)),
        };
        // throw error if credentials are missing
        if token.is_none() {
            return match request.service {
                Service::NadeoServices | Service::NadeoLiveServices => {
                    Err(Error::from(ClientError::MissingNormalAuth))
                }
                Service::OAuth => Err(Error::from(ClientError::MissingOAuth)),
            };
        }

        let mut headers = request.headers;
        headers.insert("Authorization", token.unwrap().parse().unwrap());

        let api_request = match request.method {
            HttpMethod::Get => self.client.get(request.url),
            HttpMethod::Post => self.client.post(request.url),
            HttpMethod::Put => self.client.put(request.url),
            HttpMethod::Patch => self.client.patch(request.url),
            HttpMethod::Delete => self.client.delete(request.url),
            HttpMethod::Head => self.client.head(request.url),
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
    MissingOAuth,
}
