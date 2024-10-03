use crate::auth::token::ParseTokenError;
use crate::Error;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Deserialized version of the refresh token from an auth request with the Nadeo API. Used for reauthentication with the Nadeo API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RefreshToken {
    secret: String,
    payload: RefreshPayload,
    signature: String,
}

impl FromStr for RefreshToken {
    type Err = Error;

    /// Deserializes the refresh token returned from the auth request.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s.split_terminator('.').collect();
        if values.len() != 3 {
            return Err(Error::Token(ParseTokenError::InvalidInput));
        }

        let secret = values[0].to_string();
        let payload_str = values[1].to_string();
        let signature = values[2].to_string();

        let payload = RefreshPayload::from_str(&payload_str)?;

        Ok(Self {
            secret,
            payload,
            signature,
        })
    }
}

impl RefreshToken {
    /// Serializes the refresh token into the format required for reauthenticating with the Nadeo API.
    pub(crate) fn encode(&self) -> String {
        format!(
            "{}.{}.{}",
            self.secret,
            self.payload.encode(),
            self.signature
        )
    }

    /// Returns the amount of **seconds** until the access token expires.
    #[allow(unused)]
    pub(crate) fn expires_in(&self) -> i64 {
        self.payload.expires_in()
    }
}

/// Deserialized version of the payload of an [`RefreshToken`].
///
/// [`RefreshToken`]: RefreshToken
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RefreshPayload {
    jti: String,
    iss: String,
    iat: i64,
    rat: i64,
    exp: i64,
    aud: String,
    usg: String,
    sid: String,
    sat: i64,
    sub: String,
    aun: String,
    rtk: bool,
    pce: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ubiservices_uid: Option<String>,
    refresh_aud: String,
    limit_type: String,
}

impl FromStr for RefreshPayload {
    type Err = Error;

    /// Deserializes the payload of a refresh token.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = BASE64_URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|e| Error::Token(ParseTokenError::from(e)))?;

        let str = String::from_iter(json.into_iter().map(|x| x as char));
        serde_json::from_str(&str).map_err(|e| Error::Token(ParseTokenError::from(e)))
    }
}

impl RefreshPayload {
    /// Serializes the payload (part of the [`RefreshToken`]) into the format required for reauthentication requests.
    ///
    /// [`RefreshToken`]: RefreshToken
    pub(crate) fn encode(&self) -> String {
        let data = serde_json::to_string(self).unwrap();

        let mut buf = String::new();
        BASE64_URL_SAFE_NO_PAD.encode_string(data, &mut buf);
        buf
    }

    /// Returns the amount of **seconds** until the refresh token expires.
    pub(crate) fn expires_in(&self) -> i64 {
        self.exp - Local::now().timestamp()
    }
}
