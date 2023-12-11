use crate::common::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use crate::Helius;


impl Helius {
    #[deprecated]
    pub fn get_token_metadata(&self, request: &TokenMetadataRequest) -> reqwest::Result<Vec<TokenMetadataResult>> {
        return self.handler.post(self.get_url_v0("token-metadata"), request);
    }
}

serializable_camel_case! {
    pub struct TokenMetadataRequest {
        pub mint_accounts: Vec<String>,
        pub include_off_chain: bool,
        pub disable_cache: bool
    }
}

// What the fuck Mert https://docs.helius.xyz/solana-apis/token-metadata-api
serializable_camel_case! {
    pub struct TokenMetadataResult {
        pub account: String,
        pub on_chain_account_info: Option<on_chain_account_info::OnChainAccountInfo>,
        pub on_chain_metadata: Option<on_chain_metadata::OnChainMetadata>,
        pub off_chain_metadata: Option<off_chain_metadata::OffChainMetadata>,
        pub legacy_metadata: Option<LegacyMetadata>
    }
}

serializable_camel_case! {
    pub struct LegacyMetadata {
        pub chain_id: Number,
        pub address: String,
        pub symbol: String,
        pub name: String,
        pub decimals: Number,
        #[serde(rename="logoURI")]
        pub logo_uri: String,
        pub tags: Vec<String>,
        pub extensions: Map<String, Value>
    }
}

// hacky use of modules to logically associate the different fields of the
// response that might have a name used elsewhere without making them long and stupid
pub mod on_chain_account_info {
    pub use super::*;

    serializable_camel_case! {
        pub struct OnChainAccountInfo {
            pub account_info: AccountInfo,
            pub error: String
        }
    }
    serializable_camel_case! {
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

    serializable_camel_case! {
        pub struct Data {
            pub parsed: Parsed,
            pub program: String,
            pub space: Number
        }
    }

    serializable_camel_case! {
        pub struct Parsed {
            pub info: Info,
            #[serde(rename="type")]
            pub parsed_type: String
        }
    }
    serializable_camel_case! {
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

    serializable_camel_case! {
        pub struct OnChainMetadata {
            pub metadata: Metadata,
            pub error: String
        }
    }

    serializable_camel_case! {
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
            pub collection_details: Option<CollectionDetails>,
            pub uses: Uses
        }
    }

    serializable_camel_case! {
        pub struct Uses {
            pub use_method: String,
            pub remaining: Number,
            pub total: Number
        }
    }

    serializable! {
        pub struct CollectionDetails {
            pub size: Number
        }
    }

    serializable! {
        pub struct Collection {
            pub key: String,
            pub verified: bool
        }
    }

    serializable_camel_case! {
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
            pub share: Number,
            pub verified: bool
        }
    }
}

pub mod off_chain_metadata {
    use super::*;
    serializable! {
        pub struct OffChainMetadata {
            pub metadata: Metadata,
            pub uri: String,
            pub error: String
        }
    }

    serializable_camel_case! {
        pub struct Metadata {
            pub name: String,
            pub symbol: String,
            pub attributes: Vec<Attribute>,
            pub seller_fee_basis_points: Number,
            pub image: String,
            pub properties: Properties
        }
    }

    serializable! {
        pub struct Properties {
            pub category: String,
            pub files: Vec<Files>,
            pub creators: Vec<Creator>
        }
    }

    serializable! {
        pub struct Creator {
            pub address: String,
            pub share: Number
        }
    }

    serializable! {
        pub struct Files {
            pub uri: String,
            #[serde(rename="type")]
            pub file_type: String
        }
    }

    serializable_camel_case! {
        pub struct Attribute {
            pub trait_type: String,
            pub value: String
        }
    }
}