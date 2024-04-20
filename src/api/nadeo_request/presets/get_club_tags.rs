use crate::api::auth::auth_info::Service;
use crate::api::nadeo_client::client::NadeoClient;
use crate::api::nadeo_request::presets::{to_list, AccountId, ClubTag};
use crate::api::nadeo_request::request::NadeoRequest;
use crate::api::nadeo_request::request_builder::HttpMethod;
use crate::make_entry;
use reqwest::header::HeaderMap;
use std::str::FromStr;
use uuid::Uuid;

impl NadeoRequest {
    pub fn get_club_tags(account_ids: &[AccountId]) -> Self {
        let ids = to_list(
            &account_ids
                .iter()
                .map(|uuid| uuid.to_string())
                .collect::<Vec<String>>(),
        );
        let url = format!(
            "https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList={}",
            ids
        );

        Self {
            service: Service::NadeoServices,
            headers: HeaderMap::new(),
            method: HttpMethod::Get,
            url,
        }
    }
}
impl NadeoClient {
    pub async fn get_club_tags(
        &mut self,
        account_ids: &[AccountId],
    ) -> anyhow::Result<Vec<(AccountId, ClubTag)>> {
        let req = NadeoRequest::get_club_tags(account_ids);
        let res = self.execute(req).await?.error_for_status()?;

        make_entry!(account_id, String, "accountId", club_tag, String, "clubTag");
        let json = res.json::<Vec<Entry>>().await?;

        let mut out = Vec::new();
        for entry in json {
            let id = Uuid::from_str(&entry.account_id)?;

            out.push((id, entry.club_tag));
        }

        Ok(out)
    }
}
