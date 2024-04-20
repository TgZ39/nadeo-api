use crate::api::auth::token_util::{Secret, Signature};
use crate::{impl_payload, impl_token};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefreshToken {
    secret: Secret,
    payload: RefreshPayload,
    signature: Signature,
}
impl_token!(RefreshToken, RefreshPayload);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefreshPayload {
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
impl_payload!(RefreshPayload, exp);
