const NADEO_SERVICES: &str = "https://prod.trackmania.core.nadeo.online/";
const NADEO_LIVE_SERVICES: &str = "https://live-services.trackmania.nadeo.live/";
const NADEO_LIVE_SERVICES_CLUB: &str = "https://meet.trackmania.nadeo.club";

enum NadeoServiceType {
    NadeoServices,
    NadeoLiveServices,
    NadeoLiveServicesClub,
}

impl NadeoServiceType {
    fn url(&self) -> String {
        match self {
            NadeoServiceType::NadeoServices => NADEO_SERVICES.to_string(),
            NadeoServiceType::NadeoLiveServices => NADEO_LIVE_SERVICES.to_string(),
            NadeoServiceType::NadeoLiveServicesClub => NADEO_LIVE_SERVICES_CLUB.to_string(),
        }
    }
}
