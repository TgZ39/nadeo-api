use crate::api::auth::auth_info::Service;
use crate::api::nadeo_request::request::{as_comma_list, NadeoRequest, ToRequest};
use crate::api::nadeo_request::request_builder::HttpMethod;
use reqwest::header::HeaderMap;
use strum::Display;

pub enum CoreRequest {
    Account(AccountRequest),
    Map(MapRequest),
    Meta(MetaRequest),
    Skin(SkinRequest),
}

// impl ToRequest for CoreRequest {
//     fn to_request(&self) -> NadeoRequest {
//         match self {
//             CoreRequest::Account(r) => { r.to_request() }
//             CoreRequest::Map(r) => {}
//             CoreRequest::Meta(r) => {}
//             CoreRequest::Skin(r) => {}
//         }
//     }
// }

pub enum AccountRequest {
    GetClubTags { account_ids: Vec<String> },
    GetWebIdentities { account_ids: Vec<String> },
    GetPlayerZones { account_ids: Vec<String> },
}

impl ToRequest for AccountRequest {
    fn to_request(&self) -> NadeoRequest {
        let url = match self {
            AccountRequest::GetClubTags { account_ids } => {
                let ids = as_comma_list(account_ids);
                format!(
                    "https://prod.trackmania.core.nadeo.online/accounts/clubTags/?accountIdList={}",
                    ids
                )
            }
            AccountRequest::GetWebIdentities { account_ids } => {
                let ids = as_comma_list(account_ids);
                format!(
                    "https://prod.trackmania.core.nadeo.online/webidentities/?accountIdList={}",
                    ids
                )
            }
            AccountRequest::GetPlayerZones { account_ids } => {
                let ids = as_comma_list(account_ids);
                format!(
                    "https://prod.trackmania.core.nadeo.online/accounts/zones/?accountIdList={}",
                    ids
                )
            }
        };
        NadeoRequest {
            url,
            method: HttpMethod::Get,
            headers: HeaderMap::new(),
            service: Service::NadeoServices,
        }
    }
}

pub enum MapRequest {
    GetMapInfo {
        map_uids: Vec<String>,
    },
    GetMapRecords {
        account_ids: Vec<String>,
        map_ids: Vec<String>,
        season_id: String,
    },
    GetRecordById {
        map_record_id: String,
    },
}

// impl ToRequest for MapRequest {
//     fn to_request(&self) -> NadeoRequest {
//         let url = match self {
//             MapRequest::GetMapInfo { map_uids } => {
//                 let ids = as_comma_list(map_uids);
//                 format!("https://prod.trackmania.core.nadeo.online/maps/?mapUidList={}", ids)
//             }
//             MapRequest::GetMapRecords { account_ids, map_ids, season_id } => {
//                 let ids = as_comma_list(account_ids);
//
//             }
//             MapRequest::GetRecordById { .. } => {}
//         };
//     }
// }

#[derive(Display)]
pub enum ApiRouteUsage {
    #[strum(to_string = "Client")]
    Client,
    #[strum(to_string = "Server")]
    Server,
}
pub enum MetaRequest {
    GetApiRoutes { usage: ApiRouteUsage },
    GetZones,
}

pub enum SkinRequest {
    GetEquippedSkins { account_ids: Vec<String> },
    GetFavoritedSkins { account_id: String },
    GetSkinInfo { skin_id: String },
}
