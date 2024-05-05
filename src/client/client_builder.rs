use crate::auth::o_auth::OAuthInfo;
use crate::auth::{AuthInfo, AuthType};
use crate::request::metadata::MetaData;
use crate::Result;
use crate::{auth, Error, NadeoClient};
use futures::future::join;
use reqwest::Client;
use thiserror::Error;

type EMail = String;
type Password = String;
type Identifier = String;
type Secret = String;

#[derive(Debug, Clone, Default)]
pub struct NadeoClientBuilder {
    normal_auth: Option<(EMail, Password)>,
    o_auth: Option<(Identifier, Secret)>,
    user_agent: Option<String>,
}

impl NadeoClientBuilder {
    /// Adds credentials for using [`AuthType::NadeoServices`] and [`AuthType::NadeoLiveServices`].
    pub fn with_normal_auth(mut self, email: &str, password: &str) -> Self {
        self.normal_auth = Some((email.to_string(), password.to_string()));

        self
    }

    /// Adds credentials for using [`AuthType::OAuth`].
    pub fn with_oauth_auth(mut self, identifier: &str, secret: &str) -> Self {
        self.o_auth = Some((identifier.to_string(), secret.to_string()));

        self
    }

    /// Adds a UserAgent which is sent along with each [`NadeoRequest`].
    /// This is required because Ubisoft blocks some default UserAgents.
    /// An example of a *good* UserAgent is:
    /// - `My amazing app / my.email.address@gmail.com`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use nadeo_api::NadeoClient;
    /// let client = NadeoClient::builder()
    ///     .with_normal_auth("my_email", "my_password")
    ///     .user_agent("API Testing / mustermann.max@gmail.com") // not a real email
    ///     .build()
    ///     .await?;
    /// ```
    ///
    /// [`NadeoRequest`]: crate::NadeoRequest
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());

        self
    }

    /// Trys to build a [`NadeoClient`].
    pub async fn build(self) -> Result<NadeoClient> {
        if self.o_auth.is_none() && self.normal_auth.is_none() {
            return Err(Error::from(NadeoClientBuilderError::MissingCredentials));
        }
        if self.user_agent.is_none() {
            return Err(Error::from(NadeoClientBuilderError::MissingUserAgent));
        }

        let meta_data = MetaData {
            user_agent: self.user_agent.unwrap(),
        };

        let client = Client::new();

        let mut normal_auth = None;
        let mut live_auth = None;

        // request normal and live auth tokens
        if let Some(auth) = self.normal_auth {
            let ticket = auth::get_ubi_auth_ticket(&auth.0, &auth.1, &meta_data, &client).await?;
            let normal_auth_fut =
                AuthInfo::new(AuthType::NadeoServices, &ticket, &meta_data, &client);
            let live_auth_fut =
                AuthInfo::new(AuthType::NadeoLiveServices, &ticket, &meta_data, &client);

            // execute 2 futures concurrently
            let (n_auth, l_auth) = join(normal_auth_fut, live_auth_fut).await;

            normal_auth = Some(n_auth?);
            live_auth = Some(l_auth?);
        }

        let mut o_auth = None;

        // request oauth token
        if let Some(auth) = self.o_auth {
            let auth = OAuthInfo::new(&auth.0, &auth.1, &client).await?;

            o_auth = Some(auth)
        }

        Ok(NadeoClient {
            client,
            normal_auth,
            live_auth,
            o_auth,
            meta_data,
        })
    }
}

#[derive(Error, Debug)]
pub enum NadeoClientBuilderError {
    #[error("No credentials were provided. At least 1 auth method is required")]
    MissingCredentials,
    #[error("No UserAgent was provided")]
    MissingUserAgent,
}
