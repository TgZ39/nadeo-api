use crate::auth::token::access_token::AccessToken;
use crate::auth::token::refresh_token::RefreshToken;
use crate::client::{EXPIRATION_TIME_BUFFER, NADEO_REFRESH_URL};
use crate::{Error, Result};
use reqwest::header::{HeaderMap, USER_AGENT};
use reqwest::Client;
use serde_json::{json, Value};
use std::str::FromStr;

pub mod token;

#[derive(strum::Display, Clone, Debug)]
pub enum Service {
    #[strum(to_string = "NadeoServices")]
    NadeoServices,
    #[strum(to_string = "NadeoLiveServices")]
    NadeoLiveServices,
}

#[derive(Debug, Clone)]
pub(crate) struct AuthInfo {
    pub service: Service,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl AuthInfo {
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

    pub(crate) async fn refresh(&mut self, client: &Client) -> Result<bool> {
        if !self.expires_in() < EXPIRATION_TIME_BUFFER {
            return Ok(false);
        }

        self.force_refresh(client).await.map(|_| true)
    }

    pub(crate) fn expires_in(&self) -> i64 {
        self.access_token.expires_in()
    }
}
