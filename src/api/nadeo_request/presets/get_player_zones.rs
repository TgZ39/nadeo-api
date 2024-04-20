use crate::api::auth::auth_info::Service;
use crate::api::nadeo_client::client::NadeoClient;
use crate::api::nadeo_request::presets::{to_list, AccountId, ZoneId};
use crate::api::nadeo_request::request::NadeoRequest;
use crate::api::nadeo_request::request_builder::HttpMethod;
use crate::make_entry;
use reqwest::header::HeaderMap;
use std::str::FromStr;

impl NadeoRequest {
    pub fn get_player_zones(account_ids: &[AccountId]) -> Self {
        let ids = to_list(
            &account_ids
                .iter()
                .map(|uuid| uuid.to_string())
                .collect::<Vec<_>>(),
        );
        let url = format!(
            "https://prod.trackmania.core.nadeo.online/accounts/zones/?accountIdList={}",
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

impl NadeoClient {
    pub async fn get_player_zones(
        &mut self,
        account_ids: &[AccountId],
    ) -> anyhow::Result<Vec<(AccountId, ZoneId)>> {
        let req = NadeoRequest::get_player_zones(account_ids);
        let res = self.execute(req).await?.error_for_status()?;

        make_entry!(account_id, String, "accountId", zone_id, String, "zoneId");
        let json = res.json::<Vec<Entry>>().await?;

        let mut out = Vec::new();
        for entry in json {
            let account_id = AccountId::from_str(&entry.account_id)?;
            let zone_id = ZoneId::from_str(&entry.zone_id)?;

            out.push((account_id, zone_id));
        }

        Ok(out)
    }
}
