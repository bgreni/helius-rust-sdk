use std::ops::Not;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Response;
use crate::{AccountWebhookEncoding, CollectionIdentifier, Helius, HeliusOptions, MintlistItem, MintlistRequest, TransactionType, TxnStatus};
use crate::common::*;

#[allow(dead_code)]
const MAX_WEBHOOK_ADDRESSES: usize = 100_000;
const WEBHOOK_BASE: &str = "webhooks";


impl Helius {
    pub fn get_all_webhooks(&self) -> reqwest::Result<Vec<Webhook>> {
        return self.handler.get(self.get_url_v0(WEBHOOK_BASE));
    }

    pub fn get_webhook_by_id(&self, webhook_id: &str) -> reqwest::Result<Webhook> {
        return self.handler.get(self.get_url_v0(format!("{WEBHOOK_BASE}/{webhook_id}").as_str()));
    }

    pub fn create_webhook(&self, request: &CreateWebhookRequest) -> reqwest::Result<Webhook> {
        return self.handler.post(self.get_url_v0(WEBHOOK_BASE), request);
    }

    pub fn edit_webhook(&self, request: &EditWebhookRequest) -> reqwest::Result<Webhook> {
        return self.handler.put(
            self.get_url_v0(format!("{WEBHOOK_BASE}/{}", request.webhook_id).as_str()),
            &request.data
        );
    }

    pub fn delete_webhook(&self, webhook_id: &str) -> reqwest::Result<Response> {
        return self.handler.delete(self.get_url_v0(format!("{WEBHOOK_BASE}/{}", webhook_id).as_str()));
    }

    pub fn append_addresses_to_webhook(&self, webhook_id: &str, new_addresses: Vec<String>) -> reqwest::Result<Webhook> {
        let mut webhook = self.get_webhook_by_id(webhook_id)?;
        webhook.webhook_data.account_addresses.extend(new_addresses);
        let edit_request = EditWebhookRequest {
            webhook_id: webhook_id.to_string(),
            data: webhook.webhook_data,
        };
        return self.edit_webhook(&edit_request);
    }

    pub fn create_collection_webhook(&self, request: &CreateCollectionWebhookRequest) -> reqwest::Result<Webhook> {
        let mint_request = MintlistRequest {
            query: request.collection_query.clone(),
            options: Some(HeliusOptions{limit: Some(10000), pagination_token: None}),
        };
        let mut mintlist = Vec::<MintlistItem>::new();
        let mut mints = self.get_mintlist(&mint_request)?;

        mintlist.extend(mints.result);

        while mints.pagination_token.clone().is_empty().not() {
            mints = self.get_mintlist(&MintlistRequest {
                query: request.collection_query.clone(),
                options: Some(HeliusOptions {
                    limit: mint_request.options.clone().unwrap().limit,
                    pagination_token: Some(mints.pagination_token)
                })
            })?;
            mintlist.extend(mints.result);
        }

        let data = request.data.clone();
        let webhook_request = CreateWebhookRequest {
            data: WebhookData {
                webhook_url: data.webhook_url,
                transaction_types: data.transaction_types,
                account_addresses: mintlist.iter().map(|item| return item.mint.clone()).collect(),
                webhook_type: data.webhook_type,
                auth_header: data.auth_header,
                txn_status: data.txn_status,
                encoding: data.encoding
            },
        };

        return self.create_webhook(&webhook_request);
    }
}

serializable_camel_case! {
    pub struct Webhook {
        #[serde(rename="webhookID")]
        pub webhook_id: String,
        pub wallet: String,
        #[serde(flatten)]
        pub webhook_data: WebhookData
    }
}

serializable_camel_case! {
    pub struct WebhookData {
        #[serde(rename="webhookURL")]
        pub webhook_url: String,
        pub transaction_types: Vec<TransactionType>,
        pub account_addresses: Vec<String>,
        pub webhook_type: Option<WebhookType>,
        pub auth_header: Option<String>,
        pub txn_status: Option<TxnStatus>,
        pub encoding: Option<AccountWebhookEncoding>
    }
}

enum_serializable! {
    pub enum WebhookType {
        #[serde(rename="enhanced")]
        Enhanced,
        #[serde(rename="enhancedDevnet")]
        EnhanvedDevnet,
        #[serde(rename="raw")]
        Raw,
        #[serde(rename="rawDevnet")]
        RawDevnet,
        #[serde(rename="discord")]
        Discord,
        #[serde(rename="discordDevnet")]
        DiscordDevnet
    }
}

serializable! {
    pub struct CreateWebhookRequest {
        #[serde(flatten)]
        pub data: WebhookData,
    }
}

serializable_camel_case! {
    pub struct CreateCollectionWebhookRequest {
        #[serde(flatten)]
        pub data: WebhookData,
        pub collection_query: CollectionIdentifier
    }
}

serializable! {
    pub struct EditWebhookRequest {
        pub webhook_id: String,
        pub data: WebhookData,
    }
}