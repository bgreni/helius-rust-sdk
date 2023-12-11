use crate::common::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;


serializable! {
    pub struct Pagination {
        pub page: u32,
        pub limit: Option<u32>,
        pub before: Option<u32>,
        pub after: Option<u32>,
    }
}

serializable_camel_case! {
    pub struct SearchAssetsParams {
        #[serde(flatten)]
        pub pagination: Pagination,
        pub display_options: Option<DisplayOptions>,
        pub sort_by: Option<AssetSortingRequest>,
        pub creator_address: Option<String>,
        pub owner_address: Option<String>,
        pub json_uri: Option<String>,
        pub grouping: Option<Vec<String>>,
        pub burnt: Option<bool>,
        pub frozen: Option<bool>,
        pub supply_mint: Option<String>,
        pub supply: Option<u32>,
        pub interface: Option<Interface>,
        pub delegate: Option<u32>,
        pub owner_type: Option<OwnershipModel>,
        pub royalty_amount: Option<u32>,
        pub royalty_target: Option<String>,
        pub royalty_target_type: Option<RoyaltyModel>,
        pub compressible: Option<bool>,
        pub compressed: Option<bool>
    }
}

serializable_camel_case! {
    pub struct GetAssetsByGroupParams {
        pub group_value: String,
        pub group_key: String,
        #[serde(flatten)]
        pub pagination: Pagination,
        pub display_options: Option<DisplayOptions>,
        pub sort_by: Option<AssetSortingRequest>

    }
}

serializable_camel_case! {
    pub struct GetAssetsByCreatorParams {
        pub creator_address: String,
        #[serde(flatten)]
        pub pagination: Pagination,
        pub only_verified: Option<bool>,
        pub display_options: Option<DisplayOptions>,
        pub sort_by: Option<AssetSortingRequest>
    }
}

serializable_camel_case! {
    pub struct GetAssetsByAuthorityParams {
        pub authority_address: String,
        #[serde(flatten)]
        pub pagination: Pagination,
        pub display_options: Option<DisplayOptions>,
        pub sort_by: Option<AssetSortingRequest>
    }
}

serializable_camel_case! {
    pub struct GetAssetsByOwnerParams {
        pub owner_address: String,
        #[serde(flatten)]
        pub pagination: Pagination,
        pub display_options: Option<DisplayOptions>,
        pub sort_by: Option<AssetSortingRequest>
    }
}

serializable_camel_case! {
    pub struct AssetSortingRequest {
        pub sort_by: AssetSortBy,
        pub sort_direction: AssetSortDirection
    }
}

serializable! {
    pub struct GetAssetProofParams {
        pub id: String
    }
}

serializable! {
    pub struct GetAssetProofBatchParams {
        pub ids: Vec<String>
    }
}

serializable! {
    pub struct GetAssetProofResponse {
        pub root: String,
        pub proof: Vec<String>,
        pub node_index: u32,
        pub leaf: String,
        pub tree_id: String
    }
}

serializable! {
    pub struct GetAssetParams {
        pub id: String,
        #[serde[rename = "displayOptions"]]
        pub display_options: Option<DisplayOptions>
    }
}

serializable! {
    pub struct GetAssetBatchParams {
        pub ids: Vec<String>,
        #[serde[rename = "displayOptions"]]
        pub display_options: Option<DisplayOptions>
    }
}

serializable! {
    pub struct GetAssetResponse {
        pub interface: Interface,
        pub id: String,
        pub content: Option<Content>,
        pub authorities: Option<Vec<Authorities>>,
        pub compression: Option<Compression>,
        pub grouping: Option<Vec<Grouping>>,
        pub royalty: Option<Royalty>,
        pub ownership: Ownership,
        pub creators: Option<Vec<Creators>>,
        pub uses: Option<Uses>,
        pub supply: Option<Supply>,
        pub mutable: bool,
        pub burnt: bool
    }
}

serializable! {
    pub struct GetAssetResponseList {
        pub grand_total: Option<bool>,
        pub total: u32,
        pub limit: u32,
        pub page: u32,
        pub items: Vec<GetAssetResponse>
    }
}

serializable_camel_case! {
    pub struct DisplayOptions {
        pub show_fungible: Option<bool>,
        pub show_inscription: Option<bool>
    }
}

serializable! {
    pub struct Ownership {
        pub frozen: bool,
        pub delegated: bool,
        pub delegate: Option<String>,
        pub ownership_model: OwnershipModel,
        pub owner: String
    }
}

serializable! {
    pub struct Supply {
        pub print_max_supply: u32,
        pub print_current_supply: u32,
        pub edition_nonce: Option<u32>
    }
}
serializable! {
    pub struct Uses {
        pub use_method: UseMethods,
        pub remaining: u32,
        pub total: u32
    }
}

serializable! {
    pub struct Creators {
        pub address: String,
        pub share: u32,
        pub verified: bool
    }
}

serializable! {
    pub struct Royalty {
        pub royalty_model: RoyaltyModel,
        pub target: Option<String>,
        pub percent: f32,
        pub basis_points: u32,
        pub primary_sale_happened: bool,
        pub locked: bool
    }
}

serializable! {
    pub struct Grouping {
        pub group_key: String,
        pub group_value: String,
        pub verified: Option<bool>,
        pub collection_metadata: Option<CollectionMetadata>
    }
}

serializable! {
    pub struct CollectionMetadata {
        pub name: Option<String>,
        pub symbol: Option<String>,
        pub image: Option<String>,
        pub description: Option<String>,
        pub external_url: Option<String>
    }
}

serializable! {
    pub struct Authorities {
        pub address: String,
        pub scopes: Vec<Scope>
    }
}

serializable! {
    pub struct Links {
        pub external_url: Option<String>,
        pub image: Option<String>,
        pub animation_url: Option<String>
    }
}

serializable! {
    pub struct Content {
        #[serde(rename = "$schema")]
        pub schema: String,
        pub json_uri: String,
        pub files: Option<Vec<File>>,
        pub metadata: Metadata,
        pub links: Links
    }
}

serializable! {
    pub struct File {
        pub uri: Option<String>,
        pub mime: Option<String>,
        pub cdn_uri: Option<String>,
        pub quality: Option<FileQuality>,
        pub contexts: Option<Vec<Context>>
    }
}

serializable! {
    pub struct FileQuality {
        pub schema: String
    }
}

serializable! {
    pub struct Metadata {
        pub attributes: Option<Vec<Attribute>>,
        pub description: String,
        pub name: String,
        pub symbol: String
    }
}

serializable! {
    pub struct Attribute {
        pub value: Value,
        pub trait_type: String
    }
}

serializable! {
    pub struct Compression {
        pub eligible: bool,
        pub compressed: bool,
        pub data_hash: String,
        pub creator_hash: String,
        pub asset_hash: String,
        pub tree: String,
        pub seq: u32,
        pub leaf_id: u32
    }
}