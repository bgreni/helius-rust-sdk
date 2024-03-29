use crate::common::*;
use crate::{CollectionIdentifier, Helius, HeliusOptions};
use serde::{Deserialize, Serialize};


impl Helius {
    #[deprecated]
    pub fn get_mintlist(&self, request: &MintlistRequest) -> reqwest::Result<MintlistResponse> {
        return self.handler.post(self.get_url_v1("mintlist"), request);
    }
}

serializable! {
    pub struct MintlistRequest {
        pub query: CollectionIdentifier,
        pub options: Option<HeliusOptions>
    }
}

serializable_camel_case! {
    pub struct MintlistResponse {
        pub result: Vec<MintlistItem>,
        pub pagination_token: String
    }
}

serializable! {
    pub struct MintlistItem {
        pub mint: String,
        pub name: String
    }
}
