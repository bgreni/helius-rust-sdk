mod balances;
mod enhanced_transactions;
mod name;
mod nft;
mod rpc_client;
mod token_metadata;
mod webhook;

pub use {
    balances::*, enhanced_transactions::*, name::*, nft::*, rpc_client::*, token_metadata::*,
    webhook::*,
};

use crate::{common::Cluster, util::rpc_url_from_cluster};
use solana_client::rpc_client::RpcClient;

const API_URL_V1: &str = "https://api.helius.xyz/v1";
const API_URL_V0: &str = "https://api.helius.xyz/v0";
const DEV_API_URL_V0: &str = "https://api-devnet.helius.xyz/v0";

pub struct Helius {
    api_key: String,
    cluster: Cluster,
    http_client: reqwest::blocking::Client,
    pub rpc: HeliusRpc,
}

impl Helius {
    pub fn new(api_key: String, cluster: Cluster) -> Helius {
        let endpoint = rpc_url_from_cluster(api_key.clone(), cluster);
        let connection = RpcClient::new(endpoint);
        return Helius {
            api_key,
            cluster,
            http_client: reqwest::blocking::Client::new(),
            rpc: HeliusRpc::new(connection),
        };
    }

    pub fn get_url_v1(&self, method: &str) -> String {
        return self.make_url(API_URL_V1, method);
    }

    pub fn get_url_v0(&self, method: &str) -> String {
        let url = match self.cluster {
            Cluster::MainnetBeta => API_URL_V0,
            Cluster::Devnet => DEV_API_URL_V0,
        };
        return self.make_url(url, method);
    }

    fn make_url(&self, base: &str, method: &str) -> String {
        return format!("{base}/{method}?api-key={}", self.api_key);
    }
}
