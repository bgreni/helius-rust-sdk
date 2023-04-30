use serde::{Deserialize, Serialize};
use crate::common::serializable;
use crate::Helius;
use serde_json::Number;

pub trait BalancesApi {
    fn balances(&self, address: &str) -> reqwest::Result<BalancesResponse>;
}

impl BalancesApi for Helius {
    fn balances(&self, address: &str) -> reqwest::Result<BalancesResponse> {
        let method = format!("addresses/{address}/balances");
        return self.handler.get(self.get_url_v0(method.as_str()));
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct BalancesResponse {
        tokens: Vec<TokenData>,
        native_balance: Number
    }
}


serializable! {
    #[serde(rename_all="camelCase")]
    pub struct TokenData {
        mint: String,
        amount: Number,
        decimals: Number,
        token_account: String
    }
}