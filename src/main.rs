mod access_token;
mod auth;
mod client;
mod api {
    mod client;
    mod service_client;
    mod live_service_client;
    mod request;
}

use crate::client::ServiceClient;
use base64::Engine;

const UBISOFT_AUTH_URL: &str = "https://public-ubiservices.ubi.com/v3/profiles/sessions";
const EMAIL: &str = "badbaboimbus+ubisoft@gmail.com";
const PASSWORD: &str = ":Fwi#^*v8b#ue#jv";
const USER_AGENT: &str = "Testing the API / badbaboimbus+ubisoft@gmail.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let mut client = ServiceClient::new().await?;

    Ok(())
}
