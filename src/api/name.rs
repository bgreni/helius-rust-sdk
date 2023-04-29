use serde::{Deserialize, Serialize};
use crate::common::serializable;
use crate::Helius;

pub trait NameApi {
    fn get_names(&self, address: &str) -> reqwest::Result<Names>;
}

impl NameApi for Helius {
    fn get_names(&self, address: &str) -> reqwest::Result<Names> {
        let method = format!("addresses/{address}/names");
        return self.http_client
            .get(self.get_url_v0(method.as_str()))
            .send()?.error_for_status()?
            .json::<Names>();
    }
}

serializable! {
    #[serde(rename_all="camelCase")]
    pub struct Names {
        domain_names: Vec<String>
    }
}