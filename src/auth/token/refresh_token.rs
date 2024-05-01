use crate::auth::token::TokenError;
use crate::Error;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RefreshToken {
    secret: String,
    payload: RefreshPayload,
    signature: String,
}

impl FromStr for RefreshToken {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s.split_terminator('.').collect();
        if values.len() != 3 {
            return Err(Error::Token(TokenError::InvalidInput));
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
    pub(crate) fn encode(&self) -> String {
        format!(
            "{}.{}.{}",
            self.secret,
            self.payload.encode(),
            self.signature
        )
    }

    #[allow(unused)]
    pub(crate) fn expires_in(&self) -> i64 {
        self.payload.expires_in()
    }
}

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
    ubiservices_uid: String,
    refresh_aud: String,
    limit_type: String,
}

impl FromStr for RefreshPayload {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json = BASE64_URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|e| Error::Token(TokenError::from(e)))?;

        let str = String::from_iter(json.into_iter().map(|x| x as char));
        serde_json::from_str(&str).map_err(|e| Error::Token(TokenError::from(e)))
    }
}

impl RefreshPayload {
    pub(crate) fn encode(&self) -> String {
        let data = serde_json::to_string(self).unwrap();

        let mut buf = String::new();
        BASE64_URL_SAFE_NO_PAD.encode_string(data, &mut buf);
        buf
    }

    pub(crate) fn expires_in(&self) -> i64 {
        Local::now().timestamp() - self.exp
    }
}
