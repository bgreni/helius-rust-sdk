use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub struct HeliusRpc {
    connection: RpcClient
}

impl HeliusRpc {
    pub fn new(connection: RpcClient) -> HeliusRpc {
        return HeliusRpc {
            connection
        }
    }

    pub fn connection(&self) -> &RpcClient {
        return &self.connection;
    }

    pub fn get_tps(&self) -> Result<f32, String> {
        let res = self.connection.get_recent_performance_samples(Some(1));
        return match res {
            Ok(samples) =>
                Ok(samples[0].num_transactions as f32 / samples[0].sample_period_secs as f32),
            _ => Err("Failed to get performance samples".to_string())
        }
    }

    pub fn airdrop(
        &self,
        pubkey: &Pubkey,
        lamports: u64
    ) -> solana_client::client_error::Result<()> {
        let signature = self.connection.request_airdrop(pubkey, lamports)?;
        return self.connection.poll_for_signature(&signature);
    }
}