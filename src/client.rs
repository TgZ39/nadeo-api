use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::Value;

use crate::access_token::{AccessToken, RefreshToken};
use crate::{EMAIL, PASSWORD, UBISOFT_AUTH_URL, USER_AGENT};

const NADEO_AUTH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/ubiservices";
const NADEO_REFRESH_URL: &str =
    "https://prod.trackmania.core.nadeo.online/v2/authentication/token/refresh";

const UBISOFT_APP_ID: &str = "86263886-327a-4328-ac69-527f0d20a237";

const EXPIRATION_TIME_BUFFER: i64 = 60;

pub struct ServiceClient {
    pub client: Client,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl ServiceClient {
    pub async fn new() -> anyhow::Result<ServiceClient> {
        let client = Client::default();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Ubi-AppId", UBISOFT_APP_ID.parse().unwrap());
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());

        let auth_token = {
            let auth = format!("{}:{}", EMAIL, PASSWORD);
            let auth = auth.as_bytes();

            let mut b64 = String::new();
            BASE64_STANDARD.encode_string(auth, &mut b64);

            format!("Basic {b64}")
        };
        headers.insert("Authorization", auth_token.parse().unwrap());

        // get ubisoft ticket
        let res = client
            .post(UBISOFT_AUTH_URL)
            .headers(headers)
            .send()
            .await?;

        let json = res.json::<Value>().await?;
        let ticket = json["ticket"].as_str().unwrap().to_string();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let auth_token = format!("ubi_v1 t={}", ticket);
        headers.insert("Authorization", auth_token.parse().unwrap());

        // get nadeo tokens
        let res = client.post(NADEO_AUTH_URL).headers(headers).send().await?;
        let json = res.json::<Value>().await?;

        let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
        let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());

        let auth_token = format!("nadeo_v1 t={}", access_token.encode());
        headers.insert("Authorization", auth_token.parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        Ok(ServiceClient {
            client,
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh_token(&mut self) -> anyhow::Result<()> {
        let mut headers = HeaderMap::new();

        let auth_token = format!("nadeo_v1 t={}", self.refresh_token.encode());
        headers.insert("Authorization", auth_token.parse().unwrap());

        let res = self
            .client
            .post(NADEO_REFRESH_URL)
            .headers(headers)
            .send()
            .await?;
        let json = res.json::<Value>().await?;

        let access_token = AccessToken::from_str(json["accessToken"].as_str().unwrap())?;
        let refresh_token = RefreshToken::from_str(json["refreshToken"].as_str().unwrap())?;

        self.refresh_token = refresh_token;
        self.access_token = access_token;

        Ok(())
    }

    pub async fn get_club_tags(&mut self, account_ids: Vec<&str>) -> anyhow::Result<Vec<String>> {
        let account_ids_formatted = {
            let mut out = String::with_capacity(account_ids.len() * 37);

            for id in &account_ids {
                out.push_str(id);
                out.push(',');
            }
            // remove trailing comma
            out.pop();

            out
        };

        let url = format!(
            "https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList={}",
            account_ids_formatted
        );

        if self.access_token.expires_in() < EXPIRATION_TIME_BUFFER {
            self.refresh_token().await?;
        }
        let res = self.client.get(url).send().await?;
        let json = res.json::<Value>().await?;

        let mut out = Vec::with_capacity(account_ids.len());
        for obj in json.as_array().unwrap() {
            out.push(obj["clubTag"].as_str().unwrap().to_string())
        }

        Ok(out)
    }
}

fn get_ubisoft_auth_token() -> String {
    let auth = format!("{}:{}", EMAIL, PASSWORD);
    let auth = auth.as_bytes();

    let mut b64 = String::new();
    BASE64_STANDARD.encode_string(auth, &mut b64);

    format!("Basic {b64}")
}
