use crate::auth;
use crate::auth::oauth::OAuthInfo;
use crate::auth::{AuthInfo, AuthType};
use crate::client::NadeoClient;
use crate::error::*;
use crate::request::metadata::MetaData;
use futures::future::join3;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

type EMail = String;
type Username = String;
type Password = String;
type Identifier = String;
type Secret = String;

#[derive(Debug, Clone, Default)]
pub struct NadeoClientBuilder {
    normal_auth: Option<(EMail, Password)>,
    server_auth: Option<(Username, Password)>,
    o_auth: Option<(Identifier, Secret)>,
    user_agent: Option<String>,
}

impl NadeoClientBuilder {
    /// Adds credentials for using [`AuthType::NadeoServices`] and [`AuthType::NadeoLiveServices`].
    /// Email and password of an ubisoft account are required.
    pub fn with_normal_auth(mut self, email: &str, password: &str) -> Self {
        self.normal_auth = Some((email.to_string(), password.to_string()));

        self
    }

    /// Adds credentials for using [`AuthType::NadeoServices`] and [`AuthType::NadeoLiveServices`] using a server account.
    /// [`NadeoClientBuilder`] will prefer [`NadeoClientBuilder::with_normal_auth`] if `with_normal_auth` and `with_server_auth` are added.
    pub fn with_server_auth(mut self, username: &str, password: &str) -> Self {
        self.server_auth = Some((username.to_string(), password.to_string()));

        self
    }

    /// Adds credentials for using [`AuthType::OAuth`].
    pub fn with_oauth(mut self, identifier: &str, secret: &str) -> Self {
        self.o_auth = Some((identifier.to_string(), secret.to_string()));

        self
    }

    /// Adds a UserAgent which is sent along with each [`NadeoRequest`].
    /// This is required because Ubisoft blocks some default UserAgents.
    /// An example of a *good* UserAgent is: `My amazing app / my.email.address@gmail.com`
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());

        self
    }

    /// Tries to build a [`NadeoClient`].
    pub async fn build(self) -> Result<NadeoClient> {
        if self.o_auth.is_none() && self.normal_auth.is_none() && self.server_auth.is_none() {
            return Err(Error::from(NadeoClientBuilderError::MissingCredentials));
        }
        if self.user_agent.is_none() {
            return Err(Error::from(NadeoClientBuilderError::MissingUserAgent));
        }

        let meta_data = MetaData {
            user_agent: self.user_agent.unwrap(),
        };

        let client = Client::new();

        // Ubisoft auth ticket
        let mut ticket = String::new();
        if let Some(ref auth) = self.normal_auth {
            ticket = auth::get_ubi_auth_ticket(&auth.0, &auth.1, &meta_data, &client).await?;
        }

        // NadeoServices
        let normal_auth_future = async {
            if self.normal_auth.is_some() {
                Some(AuthInfo::new(AuthType::NadeoServices, &ticket, &meta_data, &client).await)
            } else if let Some((ref username, ref password)) = self.server_auth {
                Some(
                    AuthInfo::new_server(
                        AuthType::NadeoServices,
                        &meta_data,
                        username,
                        password,
                        &client,
                    )
                    .await,
                )
            } else {
                None
            }
        };
        // NadeoLiveServices
        let live_auth_future = async {
            if self.normal_auth.is_some() {
                Some(AuthInfo::new(AuthType::NadeoLiveServices, &ticket, &meta_data, &client).await)
            } else if let Some((ref username, ref password)) = self.server_auth {
                Some(
                    AuthInfo::new_server(
                        AuthType::NadeoLiveServices,
                        &meta_data,
                        username,
                        password,
                        &client,
                    )
                    .await,
                )
            } else {
                None
            }
        };
        // OAuth
        let oauth_future = async {
            if let Some(auth) = self.o_auth {
                Some(OAuthInfo::new(&auth.0, &auth.1, &client).await)
            } else {
                None
            }
        };
        // execute requests
        let (normal_auth_res, live_auth_res, oauth_res) =
            join3(normal_auth_future, live_auth_future, oauth_future).await;

        let mut normal_auth = None;
        let mut live_auth = None;
        let mut o_auth = None;

        // extract results
        if let Some(auth) = normal_auth_res {
            normal_auth = Some(Arc::new(auth?));
        }
        if let Some(auth) = live_auth_res {
            live_auth = Some(Arc::new(auth?));
        }
        if let Some(auth) = oauth_res {
            o_auth = Some(Arc::new(auth?));
        }

        Ok(NadeoClient {
            client,
            normal_auth,
            live_auth,
            o_auth,
            meta_data: Arc::new(meta_data),
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
