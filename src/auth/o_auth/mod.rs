use crate::client::EXPIRATION_TIME_BUFFER;
use crate::Result;
use chrono::Local;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const O_AUTH_URL: &str = "https://api.trackmania.com/api/access_token";

/// Contains information used for OAuth authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OAuthInfo {
    #[serde(skip)]
    identifier: String,
    #[serde(skip)]
    secret: String,
    pub(crate) token_type: String,
    #[serde(skip)]
    pub(crate) exp: i64,
    pub(crate) access_token: String,
}

impl OAuthInfo {
    pub(crate) async fn new(identifier: &str, secret: &str, client: &Client) -> Result<Self> {
        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");
        form.insert("client_id", identifier);
        form.insert("client_secret", secret);

        let res = client
            .post(O_AUTH_URL)
            .form(&form)
            .send()
            .await?
            .error_for_status()?;

        let mut json = res.json::<Self>().await?;
        json.exp = Local::now().timestamp();
        json.identifier = identifier.to_string();
        json.secret = secret.to_string();

        Ok(json)
    }

    pub(crate) async fn force_refresh(&mut self, client: &Client) -> Result<()> {
        let new = Self::new(&self.identifier, &self.secret, client).await?;
        self.token_type = new.token_type;
        self.access_token = new.access_token;
        self.secret = new.secret;

        Ok(())
    }

    pub(crate) async fn refresh(&mut self, client: &Client) -> Result<bool> {
        if self.expires_in() < EXPIRATION_TIME_BUFFER {
            self.force_refresh(client).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) fn expires_in(&self) -> i64 {
        self.exp - Local::now().timestamp()
    }
}
