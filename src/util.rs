use crate::common::Cluster;

pub fn rpc_url_from_cluster(api_key: String, cluster: Cluster) -> String {
    return match cluster {
        Cluster::MainnetBeta => format!("https://rpc.helius.xyz/?api-key={api_key}"),
        Cluster::Devnet => format!("https://rpc-devnet.helius.xyz/?api-key={api_key}")
    };
}

