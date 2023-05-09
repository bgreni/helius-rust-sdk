# Unofficial Helius SDK Written in Rust
For more in depth details about Helius features, please consult
the [official Typescript SDK](https://github.com/helius-labs/helius-sdk)

This README will largely present what functionality is available in this implementation

## Disclaimer
I'm not ready to commit to no breaking changes yet so will be making `0.x.x` releases for now

## Getting Started
### Installation And Usage
```commandline
cargo add helius-sdk
```
```rust
use helius_sdk::*;

fn main() {
    let client = Helius::new(env::var("API_KEY").unwrap(), Cluster::MainnetBeta);
}
```
## `Other` variant for Enums
Most enums in this crate have an `Other(String)` options if new variants have been added to the helius API that have
not yet been included in this crate. When using these variants, do keep in mind there is a high chance that upgrading 
to a new version will break your code if you have logic around receiving a response of this form. As they will then
have a proper distinct value in the enum for deserialization. 
## Webhooks
### Create Webhook
```rust
let hook = client.create_webhook(&CreateWebhookRequest {
        data: WebhookData {
            webhook_url: "insert url here".to_string(),
            transaction_types: vec![TransactionType::NftBid, TransactionType::NftBidCancelled],
            account_addresses: vec!["M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K".to_string()],
            webhook_type: Some(WebhookType::Discord),
            auth_header: None,
            txn_status: None,
            encoding: None
        },
    });
```
### Get All Webhooks
Retrieve all webhook for the current user account
```rust
let hooks = client.get_all_webhooks();
```
### Get Webhook by ID
```rust
let hook = client.get_webhook_by_id("insert webhook id here"));
```
### Edit Webhook
```rust
let mut hook = client.get_webhook_by_id(hook.webhook_id.as_str()).unwrap();
hook.webhook_data.webhook_type = WebhookType::Discord.into();
let ehook = client.edit_webhook(EditWebhookRequest{
    webhook_id: hook.webhook_id,
    data: hook.webhook_data,
});

```
### Delete Webhook
```rust
client.delete_webhook("insert webhook id here");
```
### Create Collection Webhook
A convenience method, not actually a part of the helius rest interface.
```rust
let res = client.create_collection_webhook(&CreateCollectionWebhookRequest {
    data: WebhookData {
        webhook_url: "insert url here".to_string(),
        transaction_types: vec![TransactionType::NftSale],
        account_addresses: vec![],
        webhook_type: Some(WebhookType::Discord),
        auth_header: None,
        txn_status: None,
        encoding: None
    },
    collection_query: CollectionIdentifier::FirstVerifiedCreators(vec!["GVkb5GuwGKydA4xXLT9PNpx63h7bhFNrDLQSxi6j5NuF".to_string()]),
});
```
## Enhanced Transactions API-
### Parse transactions
```rust
let res = client.parse_transaction(
    &ParseTransactionsRequest{
        transactions: vec!["insert txn id here".to_string()]
    }
);
```
## NFT API
### Get All Tokens For Collection
```rust
let res = client.get_mintlist(MintlistRequest {
    query: CollectionIdentifier::FirstVerifiedCreators(vec!["GVkb5GuwGKydA4xXLT9PNpx63h7bhFNrDLQSxi6j5NuF".into()]),
    options: HeliusOptions {limit: 1000.into(), pagination_token: None}.into()
});
```
## Token Metadata API
### Get Token Metadata
```rust
let res = client.get_token_metadata(&TokenMetadataRequest{
        mint_accounts: vec!["insert token mint address"],
        include_off_chain: true,
        disable_cache: false
    });
```
## RPC Abstractions
Helper methods for common RPC operations
### Get Solana TPS
```rust
client.rpc.get_tps();
```
### Request Airdrop
```rust
let key = Pubkey::new_unique();
client.rpc.airdrop(&key, 10 * LAMPORTS_PER_SOL).expect();
```
### Access Solana Connection
Users can also access the underlying solana_sdk rpc client to make 
other standard rpc calls.
```rust
let conn = client.rpc.connection();
let inflation = conn.get_inflation_rate();
```
## Todo
- Beta/Alpha endpoints?