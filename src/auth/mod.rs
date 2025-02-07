use crate::auth::token::access_token::AccessToken;
use crate::auth::token::refresh_token::RefreshToken;
use crate::client::{
    EXPIRATION_TIME_BUFFER, NADEO_AUTH_URL, NADEO_REFRESH_URL, NADEO_SERVER_AUTH_URL,
    UBISOFT_APP_ID,
};
use crate::request::metadata::MetaData;
use crate::{Error, NadeoRequest, Result};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use parking_lot::Mutex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;

pub mod oauth;
pub mod token;

const UBISOFT_AUTH_URL: &str = "https://public-ubiservices.ubi.com/v3/profiles/sessions";

/// Defines authentication credentials used for the Nadeo API.
#[derive(strum::Display, Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum AuthType {
    #[strum(to_string = "NadeoServices")]
    NadeoServices,
    #[strum(to_string = "NadeoLiveServices")]
    NadeoLiveServices,
    OAuth,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthInfo {
    pub service: AuthType,
    pub access_token: Mutex<AccessToken>,
    pub refresh_token: Mutex<RefreshToken>,
}

impl AuthInfo {
    pub(crate) async fn new(
        service: AuthType,
        ticket: &str,
        meta_data: &MetaData,
        client: &Client,
    ) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let auth_token = format!("ubi_v1 t={}", ticket);
        headers.insert("Authorization", auth_token.parse().unwrap());
        headers.insert("User-Agent", meta_data.user_agent.parse().unwrap());

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
            .await?
            .error_for_status()?;

        let json = res.json::<Value>().await?;

        let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
        let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

        Ok(Self {
            service,
            access_token: Mutex::new(access_token),
            refresh_token: Mutex::new(refresh_token),
        })
    }

    /// Create with a server account
    pub(crate) async fn new_server(
        service: AuthType,
        meta_data: &MetaData,
        username: &str,
        password: &str,
        client: &Client,
    ) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let auth_token = format!("Basic {}", encode_auth(username, password));
        headers.insert("Authorization", auth_token.parse().unwrap());
        headers.insert("User-Agent", meta_data.user_agent.parse().unwrap());

        let body = json!(
            {
                "audience": service.to_string()
            }
        );

        // get nadeo auth token
        let res = client
            .post(NADEO_SERVER_AUTH_URL)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let json = res.json::<Value>().await?;

        let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
        let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

        Ok(Self {
            service,
            access_token: Mutex::new(access_token),
            refresh_token: Mutex::new(refresh_token),
        })
    }

    /// Forces a refresh request with the Nadeo API. [`refresh`] should be preferred over `force_refresh` in most cases.
    ///
    /// [`refresh`]: AuthInfo::try_refresh
    pub(crate) async fn force_refresh(&self, meta_data: &MetaData, client: &Client) -> Result<()> {
        let mut headers = HeaderMap::new();

        // format refresh token
        let auth_token = format!("nadeo_v1 t={}", self.refresh_token.lock().encode());
        headers.insert("Authorization", auth_token.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("User-Agent", meta_data.user_agent.parse().unwrap());

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

        *self.access_token.lock() = access_token;
        *self.refresh_token.lock() = refresh_token;

        Ok(())
    }

    /// Checks whether the token is expired. If it is [`force_refresh`] is called.
    /// If the refresh was successful `Ok(true)` is returned but if it fails `Err(Error)` is returned.
    /// If the token is not expired `Ok(false)` is returned and a token refresh is not attempted.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the token is expired and the refresh request fails.
    ///
    /// [`Error`]: Error
    /// [`force_refresh`]: AuthInfo::force_refresh
    pub(crate) async fn try_refresh(&self, meta_data: &MetaData, client: &Client) -> Result<bool> {
        if !self.expires_in() < EXPIRATION_TIME_BUFFER {
            return Ok(false);
        }

        self.force_refresh(meta_data, client).await.map(|_| true)
    }

    /// Returns the amount of **seconds** until the token expires.
    pub(crate) fn expires_in(&self) -> i64 {
        self.access_token.lock().expires_in()
    }

    /// Executes a [`NadeoRequest`].
    ///
    /// # Panics
    ///
    /// Panics if the service of the [`AuthInfo`] and the [`NadeoRequest`] are not the same.
    pub(crate) async fn execute(
        &self,
        mut request: NadeoRequest,
        meta_data: &MetaData,
        client: &Client,
    ) -> Result<Response> {
        assert_eq!(self.service, request.auth_type);

        self.try_refresh(meta_data, client).await?;

        let token = format!("nadeo_v1 t={}", self.access_token.lock().encode());
        request
            .headers_mut()
            .insert("Authorization", token.parse::<HeaderValue>().unwrap());
        request.headers_mut().insert(
            "User-Agent",
            meta_data.user_agent.parse::<HeaderValue>().unwrap(),
        );

        let resp = client.execute(request.request).await?;
        Ok(resp)
    }
}

fn encode_auth(username: &str, password: &str) -> String {
    let auth = format!("{}:{}", username, password);
    let auth = auth.as_bytes();

    let mut b64 = String::new();
    BASE64_STANDARD.encode_string(auth, &mut b64);
    b64
}

pub(crate) async fn get_ubi_auth_ticket(
    email: &str,
    password: &str,
    meta_data: &MetaData,
    client: &Client,
) -> Result<String> {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Ubi-AppId", UBISOFT_APP_ID.parse().unwrap());
    headers.insert("User-Agent", meta_data.user_agent.parse().unwrap());

    let ubi_auth_token = format!("Basic {}", encode_auth(email, password));
    headers.insert("Authorization", ubi_auth_token.parse().unwrap());

    // get ubisoft ticket
    let res = client
        .post(UBISOFT_AUTH_URL)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    let json = res.json::<Value>().await?;
    let ticket = json["ticket"].as_str().unwrap().to_string();

    Ok(ticket)
}
