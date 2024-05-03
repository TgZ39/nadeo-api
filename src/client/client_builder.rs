use crate::auth::o_auth::OAuthInfo;
use crate::auth::{AuthInfo, Service};
use crate::Result;
use crate::{auth, Error, NadeoClient};
use derive_more::{Display, Error};
use futures::future::join;
use reqwest::Client;

type EMail = String;
type Password = String;
type Identifier = String;
type Secret = String;

#[derive(Debug, Clone, Default)]
pub struct NadeoClientBuilder {
    normal_auth: Option<(EMail, Password)>,
    o_auth: Option<(Identifier, Secret)>,
}

impl NadeoClientBuilder {
    /// Adds credentials for using [`Service::NadeoServices`] and [`Service::NadeoLiveServices`].
    pub fn with_normal_auth(mut self, email: &str, password: &str) -> Self {
        self.normal_auth = Some((email.to_string(), password.to_string()));

        self
    }

    /// Adds credentials for using [`Service::OAuth`].
    pub fn with_oauth_auth(mut self, identifier: &str, secret: &str) -> Self {
        self.o_auth = Some((identifier.to_string(), secret.to_string()));

        self
    }

    /// Trys to build a [`NadeoClient`].
    pub async fn build(self) -> Result<NadeoClient> {
        if self.o_auth.is_none() && self.normal_auth.is_none() {
            return Err(Error::from(NadeoClientBuilderError::MissingCredentials));
        }

        let client = Client::new();

        let mut normal_auth = None;
        let mut live_auth = None;

        // request normal and live auth tokens
        if let Some(auth) = self.normal_auth {
            let ticket = auth::get_ubi_auth_ticket(&auth.0, &auth.1, &client).await?;
            let normal_auth_fut = AuthInfo::new(Service::NadeoServices, &ticket, &client);
            let live_auth_fut = AuthInfo::new(Service::NadeoLiveServices, &ticket, &client);

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
        })
    }
}

#[derive(Error, Debug, Display)]
pub enum NadeoClientBuilderError {
    MissingCredentials,
}
