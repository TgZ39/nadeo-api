use crate::auth::token::{impl_payload, impl_token, Secret, Signature};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessToken {
    secret: Secret,
    payload: AccessPayload,
    signature: Signature,
}
impl_token!(AccessToken, AccessPayload);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessPayload {
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
impl_payload!(AccessPayload, exp);
