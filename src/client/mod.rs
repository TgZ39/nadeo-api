use crate::auth::token::access_token::AccessToken;
use crate::auth::token::refresh_token::RefreshToken;
use crate::auth::{AuthInfo, Service};
use crate::request::{HeaderMap, HttpMethod, NadeoRequest};
use crate::Result;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use futures::future::join;
use reqwest::{Client, Response};
use serde_json::{json, Value};
use std::str::FromStr;

pub(crate) const NADEO_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
pub(crate) const NADEO_REFRESH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/refresh";
pub(crate) const UBISOFT_APP_ID: &str = "86263886-327a-4328-ac69-527f0d20a237";
const UBISOFT_AUTH_URL: &str = "https://public-ubiservices.ubi.com/v3/profiles/sessions";
const USER_AGENT: &str = "Testing the API / badbaboimbus+ubisoft@gmail.com";
pub(crate) const EXPIRATION_TIME_BUFFER: i64 = 60;

/// This client can execute [`NadeoRequest`]s and handles authentication. OAuth is not supported.
///
/// # Examples
///
/// Creating a client.
/// ```rust
/// # use nadeo_api::NadeoClient;
/// let mut client = NadeoClient::new("my_email", "my_password").await?;
/// ```
///
/// [`NadeoRequest`]: NadeoRequest
#[derive(Debug, Clone)]
pub struct NadeoClient {
    pub(crate) client: Client,
    pub(crate) normal_auth: AuthInfo,
    pub(crate) live_auth: AuthInfo,
}

impl NadeoClient {
    /// Creates a new [Client](NadeoClient) and gets the authtoken for *NadeoServices* and *NadeoLiveServices*.
    ///
    /// # Errors
    ///
    /// Returns an error if there was an error while authenticating with the Nadeo API. For example when the provided E-Mail or password is incorrect.
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        let client = Client::new();

        let ticket = get_ubi_auth_ticket(email, password, &client).await?;
        let normal_auth_fut = get_nadeo_auth_token(Service::NadeoServices, &ticket, &client);
        let live_auth_fut = get_nadeo_auth_token(Service::NadeoLiveServices, &ticket, &client);

        // execute 2 futures concurrently
        let (normal_auth, live_auth) = join(normal_auth_fut, live_auth_fut).await;

        Ok(NadeoClient {
            client,
            normal_auth: normal_auth?,
            live_auth: live_auth?,
        })
    }

    /// Executes a [`NadeoRequest`] on the given [`NadeoClient`]. For more information about the API endpoints look [here](https://webservices.openplanet.dev/).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`]
    ///
    /// # Examples
    ///
    /// Gets the clubtag of a player given the *accountID*.
    /// ```rust
    /// # use nadeo_api::auth::Service;
    /// # use nadeo_api::NadeoClient;
    /// # use nadeo_api::request::{HttpMethod, NadeoRequest};
    ///
    /// // create client
    /// let mut client = NadeoClient::new("my_email", "my_password").await?;
    ///
    /// // build request
    /// let request = NadeoRequest::builder()
    ///     .url("https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList=29e75531-1a9d-4880-98da-e2acfe17c578".to_string())
    ///     .service(Service::NadeoServices)
    ///     .http_method(HttpMethod::Get)
    ///     .build()?;
    ///
    /// // execute request
    /// let response = client.execute(request).await?;
    /// ```
    ///
    /// [`Error`]: crate::Error
    /// [`NadeoRequest`]: NadeoRequest
    /// [`NadeoClient`]: NadeoClient
    pub async fn execute(&mut self, request: NadeoRequest) -> Result<Response> {
        match request.service {
            Service::NadeoServices => self.normal_auth.refresh(&self.client).await?,
            Service::NadeoLiveServices => self.live_auth.refresh(&self.client).await?,
        };

        let auth_token = {
            let token = match request.service {
                Service::NadeoServices => self.normal_auth.access_token.clone(),
                Service::NadeoLiveServices => self.live_auth.access_token.clone(),
            };

            format!("nadeo_v1 t={}", token.encode())
        };

        let mut headers = request.headers;
        headers.insert("Authorization", auth_token.parse().unwrap());

        let api_request = match request.method {
            HttpMethod::Get => self.client.get(request.url),
            HttpMethod::Post => self.client.post(request.url),
        };

        let res = api_request
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }
}

pub(crate) async fn get_ubi_auth_ticket(
    email: &str,
    password: &str,
    client: &Client,
) -> Result<String> {
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
        .await?
        .error_for_status()?;

    let json = res.json::<Value>().await?;
    let ticket = json["ticket"].as_str().unwrap().to_string();

    Ok(ticket)
}

pub(crate) async fn get_nadeo_auth_token(
    service: Service,
    ticket: &str,
    client: &Client,
) -> Result<AuthInfo> {
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
        .await?
        .error_for_status()?;

    let json = res.json::<Value>().await?;

    let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
    let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

    Ok(AuthInfo {
        service,
        access_token,
        refresh_token,
    })
}
