pub mod api {
    pub mod nadeo_client {
        pub mod client;
        pub mod client_builder;
    }
    pub mod nadeo_request {
        pub mod presets;
        pub mod request;
        pub mod request_builder;
    }
    pub mod auth {
        pub mod token {
            pub mod access_token;
            pub mod refresh_token;
        }
        pub mod auth_info;
        pub mod token_util;
    }
}
