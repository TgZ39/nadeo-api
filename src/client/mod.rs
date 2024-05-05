use crate::auth::o_auth::OAuthInfo;

use crate::auth::{AuthInfo, AuthType};
use crate::request::NadeoRequest;
use crate::{Error, Result};

use reqwest::{Client, Response};

use crate::client::client_builder::NadeoClientBuilder;
use thiserror::Error;
use crate::request::metadata::MetaData;

pub mod client_builder;

pub(crate) const NADEO_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
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
///     .with_normal_auth("email", "password") // optional (but at least 1 of the 2 is required)
///     .with_oauth_auth("identifier", "secret") // optional
///     .build()
///     .await?;
/// ```
///
/// [`NadeoRequest`]: NadeoRequest
#[derive(Debug, Clone)]
pub struct NadeoClient {
    pub(crate) client: Client,
    pub(crate) normal_auth: Option<AuthInfo>,
    pub(crate) live_auth: Option<AuthInfo>,
    pub(crate) o_auth: Option<OAuthInfo>,
    pub(crate) meta_data: MetaData
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
    /// # use nadeo_api::auth::AuthType;
    /// # use nadeo_api::NadeoClient;
    /// # use nadeo_api::request::{HttpMethod, NadeoRequest};
    ///
    /// // create client
    /// let mut client = NadeoClient::builder()
    ///     .with_normal_auth("email", "password")
    ///     .build()
    ///     .await?;
    ///
    /// // build request
    /// let request = NadeoRequest::builder()
    ///     .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578")
    ///     .auth_type(AuthType::NadeoServices)
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
        match request.auth_type {
            AuthType::NadeoServices => {
                if let Some(auth) = &mut self.normal_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::from(ClientError::MissingNadeoAuth))
                }
            }
            AuthType::NadeoLiveServices => {
                if let Some(auth) = &mut self.live_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::from(ClientError::MissingNadeoAuth))
                }
            }
            AuthType::OAuth => {
                if let Some(auth) = &mut self.o_auth {
                    auth.execute(request, &self.meta_data, &self.client).await
                } else {
                    Err(Error::from(ClientError::MissingOAuth))
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Client does not have credentials for NadeoServices or NadeoLiveServices")]
    MissingNadeoAuth,
    #[error("Client does not have OAuth credentials")]
    MissingOAuth,
}
