use crate::api::auth::auth_info::{AuthInfo, Service};
use crate::api::auth::token::access_token::AccessToken;
use crate::api::auth::token::refresh_token::RefreshToken;
use crate::api::nadeo_client::client::{NadeoClient, NADEO_AUTH_URL, UBISOFT_APP_ID};
use anyhow::Error;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::{json, Value};
use std::str::FromStr;
use thiserror::Error;

const UBISOFT_AUTH_URL: &str = "https://public-ubiservices.ubi.com/v3/profiles/sessions";
const USER_AGENT: &str = "Testing the API / badbaboimbus+ubisoft@gmail.com";

#[derive(Debug)]
pub struct NadeoClientBuilder {
    client: Client,
    email: Option<String>,
    password: Option<String>,
}

impl Default for NadeoClientBuilder {
    fn default() -> Self {
        let client = Client::new();

        NadeoClientBuilder {
            client,
            email: None,
            password: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum ClientCreationError {
    #[error("No email was provided")]
    MissingEMail,
    #[error("No password was provided")]
    MissingPassword,
}

impl NadeoClientBuilder {
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());

        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());

        self
    }

    pub async fn build(self) -> anyhow::Result<NadeoClient> {
        if self.email.is_none() {
            return Err(Error::from(ClientCreationError::MissingEMail));
        }

        if self.password.is_none() {
            return Err(Error::from(ClientCreationError::MissingPassword));
        }

        let ticket =
            get_ubi_auth_ticket(&self.email.unwrap(), &self.password.unwrap(), &self.client)
                .await?;

        let normal_auth =
            get_nadeo_auth_token(Service::NadeoServices, &ticket, &self.client).await?;
        let live_auth =
            get_nadeo_auth_token(Service::NadeoLiveServices, &ticket, &self.client).await?;

        Ok(NadeoClient {
            client: self.client,
            normal_auth,
            live_auth,
        })
    }
}

async fn get_ubi_auth_ticket(
    email: &str,
    password: &str,
    client: &Client,
) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Ubi-AppId", UBISOFT_APP_ID.parse().unwrap());
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());

    let ubi_auth_token = {
        let auth = format!("{}:{}", email, password);
        let auth = auth.as_bytes();

        let mut b64 = String::new();
        BASE64_STANDARD.encode_string(auth, &mut b64);

        format!("Basic {b64}")
    };

    headers.insert("Authorization", ubi_auth_token.parse().unwrap());

    // get ubisoft ticket
    let res = client
        .post(UBISOFT_AUTH_URL)
        .headers(headers)
        .send()
        .await?;

    let json = res.json::<Value>().await?;
    let ticket = json["ticket"].as_str().unwrap().to_string();

    Ok(ticket)
}

async fn get_nadeo_auth_token(
    service: Service,
    ticket: &str,
    client: &Client,
) -> anyhow::Result<AuthInfo> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let auth_token = format!("ubi_v1 t={}", ticket);
    headers.insert("Authorization", auth_token.parse().unwrap());
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());

    let body = json!(
        {
            "audience": service.to_string()
        }
    );

    // get nadeo auth token
    let res = client
        .post(NADEO_AUTH_URL)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    let json = res.json::<Value>().await?;

    let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
    let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

    Ok(AuthInfo {
        service,
        access_token,
        refresh_token,
    })
}
