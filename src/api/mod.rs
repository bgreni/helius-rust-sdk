mod balances;
mod enhanced_transactions;
mod name;
mod nft;
mod rpc_client;
mod token_metadata;
mod webhook;
mod das;

pub use {
    balances::*, enhanced_transactions::*, name::*, nft::*, rpc_client::*, token_metadata::*,
    webhook::*,
    das::*,
};

use crate::{common::Cluster, util::rpc_url_from_cluster};
use solana_client::rpc_client::RpcClient;
use crate::request_handler::RequestHandler;

const API_URL_V1: &str = "https://api-mainnet.helius-rpc.com/v1";
const API_URL_V0: &str = "https://api-mainnet.helius-rpc.com/v0";
const DEV_API_URL_V0: &str = "https://api-devnet.helius.xyz/v0";
const DAS_URL: &str = "https://mainnet.helius-rpc.com";

pub struct Helius {
    api_key: String,
    cluster: Cluster,
    pub rpc: HeliusRpc,
    handler: RequestHandler
}

impl Helius {
    pub fn new(api_key: String, cluster: Cluster) -> Helius {
        let endpoint = rpc_url_from_cluster(api_key.clone(), cluster);
        let connection = RpcClient::new(endpoint);
        return Helius {
            api_key,
            cluster,
            rpc: HeliusRpc::new(connection),
            handler: RequestHandler::new()
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

    pub fn get_das_url(&self) -> String {
        return self.make_url(DAS_URL, "");
    }

    fn make_url(&self, base: &str, method: &str) -> String {
        return format!("{base}/{method}?api-key={}", self.api_key);
    }
}
