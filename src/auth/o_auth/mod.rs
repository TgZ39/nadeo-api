use crate::client::EXPIRATION_TIME_BUFFER;
use crate::request::metadata::MetaData;
use crate::request::HttpMethod;
use crate::{NadeoRequest, Result};
use chrono::Local;
use reqwest::header::HeaderValue;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::auth::AuthType;

const O_AUTH_URL: &str = "https://api.trackmania.com/api/access_token";

/// Contains information used for OAuth authentication. For creating an OAuth app look [here](https://api.trackmania.com/login).
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

        let mut json = res.json::<Self>().await?;
        json.exp = Local::now().timestamp() + 3600;
        json.identifier = identifier.to_string();
        json.secret = secret.to_string();

        Ok(json)
    }

    /// Send a request to the nadeo OAuth API to get a new access token.
    pub(crate) async fn force_refresh(&mut self, client: &Client) -> Result<()> {
        let new = Self::new(&self.identifier, &self.secret, client).await?;
        self.token_type = new.token_type;
        self.access_token = new.access_token;
        self.secret = new.secret;

        Ok(())
    }

    /// Checks whether the access token is expired, if so [`OAuthInfo::force_refresh`] is called and `Ok(true)` or `Err(Error)` is returned.
    /// If the token is still valid `Ok(false)` is returned.
    pub(crate) async fn refresh(&mut self, client: &Client) -> Result<bool> {
        if self.expires_in() < EXPIRATION_TIME_BUFFER {
            self.force_refresh(client).await?;
            return Ok(true);
        }

        Ok(false)
    }

    /// Returns the amount of seconds until the token expires.
    pub(crate) fn expires_in(&self) -> i64 {
        self.exp - Local::now().timestamp()
    }

    /// Executes a [`NadeoRequest`].
    ///
    /// # Panics
    ///
    /// Panics if the `AuthType` of the request doesn't match `OAuth`.
    pub(crate) async fn execute(
        &mut self,
        request: NadeoRequest,
        meta_data: &MetaData,
        client: &Client,
    ) -> Result<Response> {
        assert_eq!(request.auth_type, AuthType::OAuth);

        self.refresh(client).await?;
        let token = format!("Bearer {}", self.access_token);

        let api_request = match request.method {
            HttpMethod::Get => client.get(request.url),
            HttpMethod::Post => client.post(request.url),
            HttpMethod::Put => client.put(request.url),
            HttpMethod::Patch => client.patch(request.url),
            HttpMethod::Delete => client.delete(request.url),
            HttpMethod::Head => client.head(request.url),
        };

        let res = api_request
            .header("Authorization", token.parse::<HeaderValue>().unwrap())
            .header(
                "User-Agent",
                meta_data.user_agent.parse::<HeaderValue>().unwrap(),
            )
            .headers(request.headers)
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }
}
