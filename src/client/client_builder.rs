use crate::Result;
use derive_more::{Display, Error};
use futures::future::join;
use reqwest::Client;
use crate::{auth, Error, NadeoClient};
use crate::auth::{AuthInfo, Service};
use crate::auth::o_auth::OAuthInfo;

type EMail = String;
type Password = String;
type Identifier = String;
type Secret = String;

pub struct NadeoClientBuilder {
    normal_auth: Option<(EMail, Password)>,
    o_auth: Option<(Identifier, Secret)>
}

impl Default for NadeoClientBuilder {
    fn default() -> Self {
        Self {
            o_auth: None,
            normal_auth: None
        }
    }
}

impl NadeoClientBuilder {
    pub fn with_normal_auth(mut self, email: &str, password: &str) -> Self {
        self.normal_auth = Some((email.to_string(), password.to_string()));

        self
    }

    pub fn with_oauth_auth(mut self, identifier: &str, secret: &str) -> Self {
        self.o_auth = Some((identifier.to_string(), secret.to_string()));

        self
    }

    pub async fn build(self) -> Result<NadeoClient> {
        if self.o_auth.is_none() && self.normal_auth.is_none() {
            return Err(Error::from(NadeoClientBuilderError::MissingCredentials));
        }

        let client = Client::new();

        let mut normal_auth = None;
        let mut live_auth = None;

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
    MissingCredentials
}