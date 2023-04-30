use std::ops::Not;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Response;
use crate::{AccountWebhookEncoding, CollectionIdentifier, Helius, HeliusOptions, MintlistItem, MintlistRequest, NftApi, TransactionType, TxnStatus};
use crate::common::serializable;

#[allow(dead_code)]
const MAX_WEBHOOK_ADDRESSES: usize = 100_000;

pub trait WebhookApi {
    // standard methods
    fn get_all_webhooks(&self) -> reqwest::Result<Vec<Webhook>>;
    fn get_webhook_by_id(&self, webhook_id: &str) -> reqwest::Result<Webhook>;
    fn create_webhook(&self, request: &CreateWebhookRequest) -> reqwest::Result<Webhook>;
    fn edit_webhook(&self, request: &EditWebhookRequest) -> reqwest::Result<Webhook>;
    fn delete_webhook(&self, webhook_id: &str) -> reqwest::Result<Response>;

    // convenience methods
    fn append_addresses_to_webhook(&self, webhook_id: &str, new_addresses: Vec<String>) -> reqwest::Result<Webhook>;
    fn create_collection_webhook(&self, request: &CreateCollectionWebhookRequest) -> reqwest::Result<Webhook>;
}

impl WebhookApi for Helius {
    fn get_all_webhooks(&self) -> reqwest::Result<Vec<Webhook>> {
        return self.http_client
            .get(self.get_url_v0("webhooks"))
            .send()?
            .error_for_status()?
            .json::<Vec<Webhook>>();
    }

    fn get_webhook_by_id(&self, webhook_id: &str) -> reqwest::Result<Webhook> {
        return self.http_client
            .get(self.get_url_v0(format!("webhooks/{webhook_id}").as_str()))
            .send()?
            .error_for_status()?
            .json::<Webhook>();
    }

    fn create_webhook(&self, request: &CreateWebhookRequest) -> reqwest::Result<Webhook> {
        return self.http_client
            .post(self.get_url_v0("webhooks"))
            .json::<CreateWebhookRequest>(request)
            .send()?
            .error_for_status()?
            .json::<Webhook>();
    }

    fn edit_webhook(&self, request: &EditWebhookRequest) -> reqwest::Result<Webhook> {
        return self.http_client
            .put(self.get_url_v0(format!("webhooks/{}", request.webhook_id).as_str()))
            .json::<WebhookData>(&request.data)
            .send()?
            .error_for_status()?
            .json::<Webhook>();
    }

    fn delete_webhook(&self, webhook_id: &str) -> reqwest::Result<Response> {
        return self.http_client
            .delete(self.get_url_v0(format!("webhooks/{}", webhook_id).as_str()))
            .send()?
            .error_for_status();
    }

    fn append_addresses_to_webhook(&self, webhook_id: &str, new_addresses: Vec<String>) -> reqwest::Result<Webhook> {
        let mut webhook = self.get_webhook_by_id(webhook_id)?;
        webhook.webhook_data.account_addresses.extend(new_addresses);
        let edit_request = EditWebhookRequest {
            webhook_id: webhook_id.to_string(),
            data: webhook.webhook_data,
        };
        return self.edit_webhook(&edit_request);
    }

    fn create_collection_webhook(&self, request: &CreateCollectionWebhookRequest) -> reqwest::Result<Webhook> {
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

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct Webhook {
        #[serde(rename="webhookID")]
        pub webhook_id: String,
        pub wallet: String,
        #[serde(flatten)]
        pub webhook_data: WebhookData
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
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

serializable! {
    #[derive(Eq, PartialEq)]
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

serializable! {
    #[serde(rename_all="camelCase")]
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