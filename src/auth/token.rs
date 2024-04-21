use derive_more::Display;
use thiserror::Error;

pub use access_token::AccessToken;
pub use refresh_token::RefreshToken;

pub mod access_token;
pub mod refresh_token;

pub type Secret = String;
pub type Signature = String;

macro_rules! impl_token {
    ($token:ty, $payload:ty) => {
        impl std::str::FromStr for $token {
            type Err = $crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $crate::auth::token::TokenError;
                use $crate::Error;

                let values: Vec<_> = s.split_terminator('.').collect();
                if values.len() != 3 {
                    return Err(Error::Token(TokenError::InvalidInput));
                }

                let secret: Secret = values[0].to_string();
                let payload_str: String = values[1].to_string();
                let signature: Signature = values[2].to_string();

                let payload = <$payload>::from_str(&payload_str)?;

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
pub(crate) use impl_token;

#[derive(Error, Display, Debug)]
pub enum TokenError {
    InvalidInput,
    Payload(#[from] PayloadError),
}

macro_rules! impl_payload {
    ($payload:ty, $exp:ident) => {
        impl std::str::FromStr for $payload {
            type Err = $crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use base64::Engine;
                use $crate::auth::token::PayloadError;
                use $crate::auth::token::TokenError;
                use $crate::Error;

                let json = base64::prelude::BASE64_URL_SAFE_NO_PAD
                    .decode(s)
                    .map_err(|e| Error::Token(TokenError::Payload(PayloadError::from(e))))?;

                let str = String::from_iter(json.into_iter().map(|x| x as char));
                serde_json::from_str(&str)
                    .map_err(|e| Error::Token(TokenError::from(PayloadError::from(e))))
            }
        }

        impl $payload {
            fn encode(&self) -> String {
                use base64::Engine;

                let data = serde_json::to_string(self).unwrap();

                let mut buf = String::new();
                base64::prelude::BASE64_URL_SAFE_NO_PAD.encode_string(data, &mut buf);

                buf
            }

            fn expires_in(&self) -> i64 {
                chrono::Local::now().timestamp() - self.$exp
            }
        }
    };
}
pub(crate) use impl_payload;

#[derive(Error, Display, Debug)]
pub enum PayloadError {
    Base64(#[from] base64::DecodeError),
    Json(#[from] serde_json::Error),
}
