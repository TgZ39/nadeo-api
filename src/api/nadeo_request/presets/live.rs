pub enum LiveRequest {
    Campaign(CampaignRequest),
}

pub enum CampaignRequest {
    GetTOTDMaps { length: u32, offset: u32 },
    GetRoyalMaps { length: u32, offset: u32 },
    GetOfficialInfo { map_uid: String },
    GetCampaigns { length: u32, offset: u32 },
}

pub enum ClubRequest {
    GetMember {
        club_id: String,
        member_id: String,
    },
    GetMemberByName {
        club_id: String,
        member_name: String,
    },
    GetActivities {
        club_id: String,
        length: u32,
        offset: u32,
        include_active_only: bool,
    },
    GetClubById {
        club_id: String,
    },
    GetClubCampaignById {
        club_id: i32,
        campaign_id: i32,
    },
}
