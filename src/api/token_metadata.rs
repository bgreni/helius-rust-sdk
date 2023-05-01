use crate::common::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use crate::Helius;

// TODO: implement proper deserialization
pub trait TokenMetadataApi {
    fn get_token_metadata(&self, request: &TokenMetadataRequest) -> reqwest::Result<Vec<Map<String, Value>>>;
}

impl TokenMetadataApi for Helius {
    fn get_token_metadata(&self, request: &TokenMetadataRequest) -> reqwest::Result<Vec<Map<String, Value>>> {
        return self.handler.post(self.get_url_v0("token-metadata"), request);
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

// What the fuck Mert https://docs.helius.xyz/solana-apis/token-metadata-api
serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TokenMetadataResult {
        pub account: String,
        pub on_chain_account_info: on_chain_account_info::OnChainAccountInfo
    }
}

// hacky use of modules to logically associate the different fields of the
// response that might have a name used elsewhere without making them long and stupid
pub mod on_chain_account_info {
    pub use super::*;

    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct OnChainAccountInfo {
            pub account_info: AccountInfo,
            pub error: String
        }
    }
    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct AccountInfo {
            pub key: String,
            pub is_signer: bool,
            pub is_writable: bool,
            pub lamports: Number,
            pub data: Data,
            pub owner: String,
            pub executable: bool,
            pub rent_epoch: Number
        }
    }

    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct Data {
            pub parsed: Parsed,
            pub program: String,
            pub space: Number
        }
    }

    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct Parsed {
            pub info: Info,
            #[serde(rename="type")]
            pub parsed_type: String
        }
    }
    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct Info {
            pub decimals: Number,
            pub freeze_authority: String,
            pub is_initialized: bool,
            pub mint_authority: String,
            pub supply: String
        }
    }
}

pub mod on_chain_metadata {
    use super::*;

    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct OnChainMetadata {
            pub metdata: Metadata
        }
    }

    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct Metadata {
            pub key: String,
            pub mint: String,
            pub update_authority: String,
            pub data: Data,
            pub token_standard: String,
            pub primary_sale_happened: bool,
            pub is_mutable: bool,
            pub edition_nonce: Number,
            pub collection: Collection,
            pub collection_details: CollectionDetails
        }
    }

    serializable! {
        pub struct CollectionDetails {

        }
    }

    serializable! {
        pub struct Collection {
            pub key: String,
            pub verified: bool
        }
    }

    serializable! {
        #[serde(rename_all="camelCase")]
        pub struct Data {
            pub name: String,
            pub symbol: String,
            pub uri: String,
            pub seller_fee_basis_points: Number,
            pub creators: Vec<Creator>,
        }
    }

    serializable! {
        pub struct Creator {
            pub address: String,
            pub share: String,
            pub verified: bool
        }
    }
}