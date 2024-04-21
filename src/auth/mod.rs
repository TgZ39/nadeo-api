use std::str::FromStr;
use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde_json::{json, Value};
use crate::client::NADEO_REFRESH_URL;

pub use token::access_token::AccessToken as AccessToken;
pub use token::refresh_token::RefreshToken as RefreshToken;

pub mod token;

#[derive(strum::Display, Clone, Debug)]
pub enum Service {
    #[strum(to_string = "NadeoServices")]
    NadeoServices,
    #[strum(to_string = "NadeoLiveServices")]
    NadeoLiveServices,
}

#[derive(Debug, Clone)]
pub struct AuthInfo {
    pub service: Service,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl AuthInfo {
    pub async fn refresh(&mut self, client: &Client) -> anyhow::Result<()> {
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
            .await?;
        let json = res.json::<Value>().await?;

        let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
        let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

        self.access_token = access_token;
        self.refresh_token = refresh_token;

        Ok(())
    }

    pub fn expires_in(&self) -> i64 {
        self.access_token.expires_in()
    }
}
