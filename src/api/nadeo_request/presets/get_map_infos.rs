use crate::api::auth::auth_info::Service;
use crate::api::nadeo_client::client::NadeoClient;
use crate::api::nadeo_request::presets::{to_list, AccountId, MapId};
use crate::api::nadeo_request::request::NadeoRequest;
use crate::api::nadeo_request::request_builder::HttpMethod;
use crate::make_entry;
use reqwest::header::HeaderMap;
use std::str::FromStr;

impl NadeoRequest {
    pub fn get_map_infos(map_uids: &[String]) -> NadeoRequest {
        let ids = to_list(map_uids);
        let url = format!(
            "https://prod.trackmania.core.nadeo.online/maps/?mapUidList={}",
            ids
        );

        NadeoRequest {
            url,
            method: HttpMethod::Get,
            headers: HeaderMap::new(),
            service: Service::NadeoServices,
        }
    }
}

#[derive(Debug)]
pub struct MapInfo {
    pub author: AccountId,
    pub author_score: i32,
    pub gold_score: i32,
    pub silver_score: i32,
    pub bronze_score: i32,
    pub collection_name: String,
    pub created_on_gamepad_editor: bool,
    pub created_on_simple_editor: bool,
    pub file_name: String,
    pub is_playable: bool,
    pub map_id: MapId,
    pub map_style: String,
    pub map_type: String,
    pub map_uid: String,
    pub name: String,
    pub submitter: AccountId,
    pub timestamp: String,
    pub file_url: String,
    pub thumbnail_url: String,
}

impl NadeoClient {
    pub async fn get_map_infos(&mut self, map_uids: &[String]) -> anyhow::Result<Vec<MapInfo>> {
        let req = NadeoRequest::get_map_infos(map_uids);
        let res = self.execute(req).await?.error_for_status()?;

        make_entry!(
            author,
            String,
            "author",
            author_score,
            i32,
            "authorScore",
            gold_score,
            i32,
            "goldScore",
            silver_score,
            i32,
            "silverScore",
            bronze_score,
            i32,
            "bronzeScore",
            collection_name,
            String,
            "collectionName",
            created_on_gamepad_editor,
            bool,
            "createdWithGamepadEditor",
            created_on_simple_editor,
            bool,
            "createdWithSimpleEditor",
            file_name,
            String,
            "filename",
            is_playable,
            bool,
            "isPlayable",
            map_id,
            String,
            "mapId",
            map_style,
            String,
            "mapStyle",
            map_type,
            String,
            "mapType",
            map_uid,
            String,
            "mapUid",
            name,
            String,
            "name",
            submitter,
            String,
            "submitter",
            timestamp,
            String,
            "timestamp",
            file_url,
            String,
            "fileUrl",
            thumbnail_url,
            String,
            "thumbnailUrl"
        );
        let json = res.json::<Vec<Entry>>().await?;

        let mut out = Vec::new();
        for entry in json {
            let info = MapInfo {
                author: AccountId::from_str(&entry.author)?,
                author_score: entry.author_score,
                gold_score: entry.gold_score,
                silver_score: entry.silver_score,
                bronze_score: entry.bronze_score,
                collection_name: entry.collection_name,
                created_on_gamepad_editor: entry.created_on_gamepad_editor,
                created_on_simple_editor: entry.created_on_simple_editor,
                file_name: entry.file_name,
                is_playable: entry.is_playable,
                map_id: MapId::from_str(&entry.map_id)?,
                map_style: entry.map_style,
                map_type: entry.map_type,
                map_uid: entry.map_uid,
                name: entry.name,
                submitter: AccountId::from_str(&entry.submitter)?,
                timestamp: entry.timestamp,
                file_url: entry.file_url,
                thumbnail_url: entry.thumbnail_url,
            };

            out.push(info);
        }

        Ok(out)
    }
}
