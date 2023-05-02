use crate::common::*;
use crate::Helius;
use serde::{Deserialize, Serialize};

pub trait NameApi {
    fn get_names(&self, address: &str) -> reqwest::Result<Names>;
}

impl NameApi for Helius {
    fn get_names(&self, address: &str) -> reqwest::Result<Names> {
        let method = format!("addresses/{address}/names");
        return self.handler.get(self.get_url_v0(method.as_str()));
    }
}

serializable_camel_case! {
    pub struct Names {
        pub domain_names: Vec<String>
    }
}
