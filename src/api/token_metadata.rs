use crate::common::serializable;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::Helius;

// TODO: implement proper deserialization
pub trait TokenMetadataApi {
    fn get_token_metadata(&self, request: &TokenMetadataRequest) -> reqwest::Result<Vec<Map<String, Value>>>;
}

impl TokenMetadataApi for Helius {
    fn get_token_metadata(&self, request: &TokenMetadataRequest) -> reqwest::Result<Vec<Map<String, Value>>> {
        return self.http_client
            .post(self.get_url_v0("token-metadata"))
            .json(request)
            .send()?
            .error_for_status()?
            .json();
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TokenMetadataRequest {
        pub mint_accounts: Vec<String>,
        pub include_off_chain: bool,
        pub disable_cache: bool
    }
}
