use serde::{Deserialize, Serialize};
use derive_alias::derive_alias;

derive_alias! {
    dataclass => #[derive(Serialize, Deserialize, Clone, Debug)]
}
pub(crate) use dataclass;

#[derive(Clone, Copy)]
pub enum Cluster {
    MainnetBeta,
    Devnet
}

dataclass! {
    pub struct HeliusOptions {
        pub limit: Option<u32>,
        #[serde(rename = "paginationToken")]
        pub pagination_token: Option<String>
    }
}

dataclass! {
    pub enum CollectionIdentifier {
        #[serde(rename = "firstVerifiedCreators")]
        FirstVerifiedCreators(Vec<String>),
        #[serde(rename = "verifiedCollectionAddress")]
        VerifiedCollectionAddress(Vec<String>)
    }
}
