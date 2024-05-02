use crate::auth::token::access_token::AccessToken;
use crate::auth::token::refresh_token::RefreshToken;
use crate::client::{EXPIRATION_TIME_BUFFER, NADEO_REFRESH_URL};
use crate::{Error, Result};
use reqwest::header::{HeaderMap, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;

pub mod token;

/// Defines Service which is used to authenticate with the Nadeo API.
#[derive(strum::Display, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Service {
    #[strum(to_string = "NadeoServices")]
    NadeoServices,
    #[strum(to_string = "NadeoLiveServices")]
    NadeoLiveServices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct AuthInfo {
    pub service: Service,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl AuthInfo {
    /// Forces a refresh request with the Nadeo API. [`refresh`] should be preferred over `force_refresh` in most cases.
    ///
    /// [`refresh`]: AuthInfo::refresh
    pub(crate) async fn force_refresh(&mut self, client: &Client) -> Result<()> {
        let mut headers = HeaderMap::new();

        // format refresh token
        let auth_token = format!("nadeo_v1 t={}", self.refresh_token.encode());
        headers.insert("Authorization", auth_token.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("User-Agent", USER_AGENT.to_string().parse().unwrap());

        let body = json!(
            {
                "audience": self.service.to_string()
            }
        );

        let res = client
            .post(NADEO_REFRESH_URL)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(Error::from)?;

        let json = res.json::<Value>().await.map_err(Error::from)?;

        let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
        let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

        self.access_token = access_token;
        self.refresh_token = refresh_token;

        Ok(())
    }
    /// Checks wether the token is expired. If it is [`force_refresh`] is called.
    /// If the refresh was successful `Ok(true)` is returned but if it fails `Err(Error)` is returned.
    /// If the token is not expired `Ok(false)` is returned and a token refresh is not attempted.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the token is expired and the refresh request fails.
    ///
    /// [`Error`]: Error
    /// [`force_refresh`]: AuthInfo::force_refresh
    pub(crate) async fn refresh(&mut self, client: &Client) -> Result<bool> {
        if !self.expires_in() < EXPIRATION_TIME_BUFFER {
            return Ok(false);
        }

        self.force_refresh(client).await.map(|_| true)
    }

    /// Returns the amount of **seconds** until the token expires.
    pub(crate) fn expires_in(&self) -> i64 {
        self.access_token.expires_in()
    }
}
