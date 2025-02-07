use crate::auth::AuthType;
use crate::client::EXPIRATION_TIME_BUFFER;
use crate::request::metadata::MetaData;
use crate::{NadeoRequest, Result};
use chrono::Local;
use parking_lot::Mutex;
use reqwest::header::HeaderValue;
use reqwest::{Client, Response};
use serde::Deserialize;
use std::cell::Cell;
use std::collections::HashMap;

const O_AUTH_URL: &str = "https://api.trackmania.com/api/access_token";

/// Contains information used for OAuth authentication. For creating an OAuth app look [here](https://api.trackmania.com/login).
#[derive(Debug, Deserialize)]
pub(crate) struct OAuthInfo {
    #[serde(skip)]
    identifier: String,
    #[serde(skip)]
    secret: String,
    #[serde(skip)]
    exp: Cell<i64>,
    access_token: Mutex<String>,
}

impl OAuthInfo {
    /// Requests access token using OAuth credentials. This function is used internally and OAuth should only be used through the [`NadeoClient`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use nadeo_api::auth::o_auth::OAuthInfo;
    ///
    /// let client = reqwest::Client::new();
    /// let info = OAuthInfo::new("your_identifier", "your_secret", &client).await?;
    /// ```
    ///
    /// [`NadeoClient`]: crate::NadeoClient
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

        let mut info = res.json::<Self>().await?;
        info.exp = Cell::new(Local::now().timestamp() + 3600);
        info.identifier = identifier.to_string();
        info.secret = secret.to_string();

        Ok(info)
    }

    /// Send a request to the Nadeo OAuth API to get a new access token.
    pub(crate) async fn force_refresh(&self, client: &Client) -> Result<()> {
        let new = Self::new(&self.identifier, &self.secret, client).await?;

        *self.access_token.lock() = new.access_token.into_inner();
        self.exp.set(new.exp.get());

        Ok(())
    }

    /// Checks whether the access token is expired, if so [`OAuthInfo::force_refresh`] is called and `Ok(true)` or `Err(Error)` is returned.
    /// If the token is still valid `Ok(false)` is returned.
    pub(crate) async fn try_refresh(&self, client: &Client) -> Result<bool> {
        if self.expires_in() < EXPIRATION_TIME_BUFFER {
            self.force_refresh(client).await?;
            return Ok(true);
        }

        Ok(false)
    }

    /// Returns the amount of seconds until the token expires.
    pub(crate) fn expires_in(&self) -> i64 {
        self.exp.get() - Local::now().timestamp()
    }

    /// Executes a [`NadeoRequest`].
    ///
    /// # Panics
    ///
    /// Panics if the `AuthType` of the request doesn't match `OAuth`.
    pub(crate) async fn execute(
        &self,
        mut request: NadeoRequest,
        meta_data: &MetaData,
        client: &Client,
    ) -> Result<Response> {
        assert_eq!(request.auth_type, AuthType::OAuth);

        self.try_refresh(client).await?;

        let token = format!("Bearer {}", self.access_token.lock());
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
