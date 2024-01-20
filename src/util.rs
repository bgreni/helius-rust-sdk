use crate::common::Cluster;

pub fn rpc_url_from_cluster(api_key: String, cluster: Cluster) -> String {
    return match cluster {
        Cluster::MainnetBeta => format!("https://devnet.helius-rpc.com/?api-key={api_key}"),
        Cluster::Devnet => format!("https://mainnet.helius-rpc.com/?api-key={api_key}"),
    };
}
