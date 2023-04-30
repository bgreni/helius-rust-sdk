#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod common;
mod util;
mod api;

pub use {
    api::*,
    common::*,
    util::*,
};

/// These shouldn't be run in normal circumstances
/// they exist exclusively so I don't have to write
/// this shit out every time I wanna test against the live service
///
/// Obviously dealing with live data we can't validate what its
/// giving us, but this serves as a sanity check that I haven't clearly
/// broken anything
#[cfg(test)]
mod tests {
    use super::*;
    struct Config {
        client: Helius,
        address: String,
        webhook_url: String,
        txn: String,
        collection: String,
        token_mint: String
    }

    impl Config {
        pub fn new() -> Config {
            return Config {
                client: Helius::new(std::env::var("API_KEY").unwrap(), Cluster::MainnetBeta),
                address: std::env::var("ADDRESS").unwrap(),
                webhook_url: std::env::var("WEBHOOK_URL").unwrap(),
                txn: std::env::var("TXN").unwrap(),
                collection: std::env::var("COL").unwrap(),
                token_mint: std::env::var("TMINT").unwrap()
            };
        }

    }

    #[test]
    #[ignore]
    fn test_balances() {
        let config = Config::new();
        assert!(config.client.balances(config.address.as_str()).is_ok())
    }

    #[test]
    #[ignore]
    fn test_webhook() {
        let config = Config::new();
        let req = CreateWebhookRequest {
            data: WebhookData {
                webhook_url: config.webhook_url,
                transaction_types: vec![TransactionType::Burn],
                account_addresses: vec![config.address],
                webhook_type: None,
                auth_header: None,
                txn_status: None,
                encoding: None,
            },
        };
        let hook_res = config.client.create_webhook(&req);
        assert!(hook_res.is_ok());
        let hook = hook_res.unwrap();
        assert_eq!(config.client.get_all_webhooks().unwrap().len(), 1);
        let mut hooky = config.client.get_webhook_by_id(hook.webhook_id.as_str()).unwrap();
        hooky.webhook_data.transaction_types.push(TransactionType::Fuse);
        let edited_hook = config.client.edit_webhook(
            &EditWebhookRequest {
                webhook_id: hooky.webhook_id.clone(),
                data: hooky.webhook_data
            });
        assert_eq!(edited_hook.unwrap().webhook_data.transaction_types, vec![TransactionType::Burn, TransactionType::Fuse]);
        assert!(config.client.delete_webhook(hook.webhook_id.as_str()).is_ok());
    }

    #[test]
    #[ignore]
    fn test_get_names() {
        let config = Config::new();
        assert!(config.client.get_names(config.address.as_str()).is_ok());
    }

    #[test]
    #[ignore]
    fn test_enhanced_txn() {
        let config = Config::new();
        assert!(config.client.parse_transaction(
            &ParseTransactionsRequest {
                transactions: vec![config.txn],
            }).is_ok());
    }

    #[test]
    #[ignore]
    fn test_get_mintlist() {
        let config = Config::new();
        assert!(config.client.get_mintlist(&MintlistRequest {
            query: CollectionIdentifier::FirstVerifiedCreators(vec![config.collection]),
            options: Some(HeliusOptions{limit: Some(10), pagination_token: None}),
        }).is_ok());
    }

    #[test]
    #[ignore]
    fn test_get_metadata() {
        let config = Config::new();
        config.client.get_token_metadata(&TokenMetadataRequest{
            mint_accounts: vec![config.token_mint],
            include_off_chain: true,
            disable_cache: false
        }).unwrap();
    }
}