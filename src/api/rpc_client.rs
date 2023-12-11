
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_client::rpc_filter::{MemcmpEncodedBytes, RpcFilterType, Memcmp};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

pub struct HeliusRpc {
    connection: RpcClient,
}

impl HeliusRpc {
    pub fn new(connection: RpcClient) -> HeliusRpc {
        return HeliusRpc { connection };
    }

    pub fn connection(&self) -> &RpcClient {
        return &self.connection;
    }

    pub fn get_tps(&self) -> Result<f32, String> {
        let res = self.connection.get_recent_performance_samples(Some(1));
        return match res {
            Ok(samples) => {
                Ok(samples[0].num_transactions as f32 / samples[0].sample_period_secs as f32)
            }
            _ => Err("Failed to get performance samples".to_string()),
        };
    }

    pub fn airdrop(
        &self,
        pubkey: &Pubkey,
        lamports: u64,
    ) -> solana_client::client_error::Result<()> {
        let signature = self.connection.request_airdrop(pubkey, lamports)?;
        return self.connection.poll_for_signature(&signature);
    }

    pub fn get_stake_accounts(&self, address: &String) -> solana_client::client_error::Result<Vec<(Pubkey, Account)>> {
        return self.connection.get_program_accounts_with_config(
            &Pubkey::from_str("Stake11111111111111111111111111111111111111").unwrap(),
            RpcProgramAccountsConfig{
                filters: Some(vec![
                    RpcFilterType::DataSize(200),
                    RpcFilterType::Memcmp(Memcmp::new(
                        44,
                        MemcmpEncodedBytes::Bytes(Vec::from(address.as_bytes()))
                    ))
                ]),
                account_config: Default::default(),
                with_context: None,
            }
        );
    }

    pub fn get_token_holders(&self, mint_address: &String) -> solana_client::client_error::Result<Vec<(Pubkey, Account)>> {
        return self.connection.get_program_accounts_with_config(
            &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            RpcProgramAccountsConfig {
                filters: Some(vec![
                    RpcFilterType::DataSize(165),
                    RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(Vec::from(mint_address.as_bytes()))
                    ))
                ]),
                account_config: Default::default(),
                with_context: None,
            }
        );
    }
}
