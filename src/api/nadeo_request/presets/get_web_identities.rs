use crate::api::auth::auth_info::Service;
use crate::api::nadeo_client::client::NadeoClient;
use crate::api::nadeo_request::presets::{to_list, AccountId};
use crate::api::nadeo_request::request::NadeoRequest;
use crate::api::nadeo_request::request_builder::HttpMethod;
use crate::make_entry;
use reqwest::header::HeaderMap;
use std::str::FromStr;
use strum::{Display, EnumString};
use uuid::Uuid;

impl NadeoRequest {
    pub fn get_web_identities(account_ids: &[AccountId]) -> Self {
        let ids = to_list(
            &account_ids
                .iter()
                .map(|uuid| uuid.to_string())
                .collect::<Vec<_>>(),
        );
        let url = format!(
            "https://prod.trackmania.core.nadeo.online/webidentities/?accountIdList={}",
            ids
        );

        Self {
            url,
            method: HttpMethod::Get,
            headers: HeaderMap::new(),
            service: Service::NadeoServices,
        }
    }
}

#[derive(EnumString, Display, Debug)]
pub enum Provider {
    #[strum(serialize = "ubiServices")]
    UbiServices,
    #[strum(serialize = "uplay")]
    UPlay,
}
impl NadeoClient {
    pub async fn get_web_identities(
        &mut self,
        account_ids: &[AccountId],
    ) -> anyhow::Result<Vec<(AccountId, Provider, Uuid)>> {
        let req = NadeoRequest::get_web_identities(account_ids);
        let res = self.execute(req).await?.error_for_status()?;

        make_entry!(
            account_id,
            String,
            "accountId",
            provider,
            String,
            "provider",
            uid,
            String,
            "uid"
        );
        let json = res.json::<Vec<Entry>>().await?;

        let mut out = Vec::new();
        for entry in json {
            let account_id = AccountId::from_str(&entry.account_id)?;
            let provider = Provider::from_str(&entry.provider)?;
            let uid = Uuid::from_str(&entry.uid)?;

            out.push((account_id, provider, uid))
        }

        Ok(out)
    }
}
