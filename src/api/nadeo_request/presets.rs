use uuid::Uuid;

pub mod get_club_tags;
pub mod get_map_infos;
pub mod get_player_zones;
pub mod get_web_identities;

pub type AccountId = Uuid;
pub type ClubTag = String;
pub type ZoneId = Uuid;
pub type MapId = Uuid;

fn to_list(list: &[String]) -> String {
    let mut out = String::new();

    for entry in list {
        out.push_str(entry);
        out.push(',')
    }
    out.pop(); // remove trailing comma

    out
}

#[macro_export]
macro_rules! make_entry {
    ($ ($field:ident, $ty:ty, $name:literal), *) => {
        #[derive(serde::Deserialize)]
        struct Entry {
            $(
                #[serde(rename(deserialize = $name ))]
                $field : $ty ,

            )*
        }
    };
}
