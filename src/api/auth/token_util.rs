use thiserror::Error;
pub type Secret = String;
pub type Signature = String;

#[macro_export]
macro_rules! impl_token {
    ($token:ty, $payload:ty) => {
        use $crate::api::auth::token_util::TokenCreationError;

        impl FromStr for $token {
            type Err = TokenCreationError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let values: Vec<_> = s.split_terminator('.').collect();
                if values.len() != 3 {
                    return Err(TokenCreationError::InvalidInput);
                }

                let secret: Secret = values[0].to_string();
                let payload_str: String = values[1].to_string();
                let signature: Signature = values[2].to_string();

                let payload = <$payload>::from_str(&payload_str)
                    .map_err(|_e| TokenCreationError::InvalidPayload)?;

                Ok(Self {
                    secret,
                    payload,
                    signature,
                })
            }
        }

        impl $token {
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
    };
}

#[derive(Error, Debug)]
pub enum TokenCreationError {
    #[error("Could not parse input")]
    InvalidInput,
    #[error("Could not parse input correctly")]
    InvalidPayload,
}

#[macro_export]
macro_rules! impl_payload {
    ($payload:ty, $exp:ident) => {
        use base64::prelude::BASE64_URL_SAFE_NO_PAD;
        use base64::Engine;
        use chrono::Local;
        use $crate::api::auth::token_util::PayloadCreationError;

        impl FromStr for $payload {
            type Err = PayloadCreationError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let json = BASE64_URL_SAFE_NO_PAD.decode(s);
                if json.is_err() {
                    return Err(PayloadCreationError::InvalidBase64);
                }

                let str = String::from_iter(json.unwrap().into_iter().map(|x| x as char));
                if let Ok(token) = serde_json::from_str(&str) {
                    return Ok(token);
                }
                Err(PayloadCreationError::InvalidJson)
            }
        }

        impl $payload {
            fn encode(&self) -> String {
                let data = serde_json::to_string(self).unwrap();

                let mut buf = String::new();
                BASE64_URL_SAFE_NO_PAD.encode_string(data, &mut buf);

                buf
            }

            fn expires_in(&self) -> i64 {
                Local::now().timestamp() - self.$exp
            }
        }
    };
}

#[derive(Debug, Error)]
pub enum PayloadCreationError {
    #[error("Input is not valid Base64")]
    InvalidBase64,
    #[error("Could not parse Base64 input correctly")]
    InvalidJson,
}
