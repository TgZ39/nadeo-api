use base64::prelude::{BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE_NO_PAD};
use base64::Engine;
use chrono::{Local, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Secret = String;
pub type Signature = String;

pub type RefreshToken = AccessToken;

#[derive(Debug)]
pub struct AccessToken {
    pub secret: Secret,
    pub payload: Payload,
    pub signature: Signature,
}

#[derive(Error, Debug)]
pub enum AccessTokenCreationError {
    #[error("Could not parse input")]
    InvalidInput,
    #[error("Could not parse input correctly")]
    InvalidPayload,
}

impl AccessToken {
    pub fn from_str(value: &str) -> Result<AccessToken, AccessTokenCreationError> {
        let values: Vec<_> = value.split_terminator('.').collect();
        if values.len() != 3 {
            return Err(AccessTokenCreationError::InvalidInput);
        }
        let secret: Secret = values[0].to_string();
        let payload_str: String = values[1].to_string();
        let signature: Signature = values[2].to_string();

        let payload = Payload::from_str(&payload_str)
            .map_err(|e| AccessTokenCreationError::InvalidPayload)?;

        Ok(AccessToken {
            secret,
            payload,
            signature,
        })
    }

    pub fn encode(&self) -> String {
        format!(
            "{}.{}.{}",
            self.secret,
            self.payload.encode(),
            self.signature
        )
    }

    pub fn expires_in(&self) -> i64 {
        self.payload.expires_in()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
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
}

#[derive(Debug, Error)]
pub enum PayloadCreationError {
    #[error("Input is not valid Base64")]
    InvalidBase64,
    #[error("Could not parse Base64 input correctly")]
    InvalidJson,
}

impl Payload {
    pub fn from_str(value: &str) -> Result<Self, PayloadCreationError> {

        let json = BASE64_URL_SAFE_NO_PAD.decode(value);
        if json.is_err() {
            return Err(PayloadCreationError::InvalidBase64);
        }

        let str = String::from_iter(json.unwrap().into_iter().map(|x| x as char));
        if let Ok(payload) = serde_json::from_str(&str) {
            return Ok(payload)
        }
        Err(PayloadCreationError::InvalidJson)
    }

    pub fn encode(&self) -> String {
        let data = serde_json::to_string(self).unwrap();

        let mut buf = String::new();
        BASE64_URL_SAFE_NO_PAD.encode_string(data, &mut buf);

        buf
    }

    pub fn expires_in(&self) -> i64 {
        Local::now().timestamp() - self.exp
    }
}
