use crate::common::serializable;
use crate::{CollectionIdentifier, Helius, HeliusOptions};
use serde::{Deserialize, Serialize};

pub trait NftApi {
    fn get_mintlist(&self, request: &MintlistRequest) -> reqwest::Result<MintlistResponse>;
}

impl NftApi for Helius {
    fn get_mintlist(&self, request: &MintlistRequest) -> reqwest::Result<MintlistResponse> {
        return self
            .http_client
            .post(self.get_url_v1("mintlist"))
            .json(request)
            .send()?
            .error_for_status()?
            .json();
    }
}

serializable! {
    pub struct MintlistRequest {
        pub query: CollectionIdentifier,
        pub options: Option<HeliusOptions>
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
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
