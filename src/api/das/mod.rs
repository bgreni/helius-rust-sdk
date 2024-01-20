mod types;

use std::collections::HashMap;
pub use {types::*};
use crate::common::*;
use serde::{Deserialize, Serialize};
use crate::Helius;

fn parse<T>(res: reqwest::Result<Res<T>>) -> reqwest::Result<T> where T: Clone {
    return res.map(|r| {return r.result});
}

impl Helius {
    pub fn get_asset(
        &self,
        params: &GetAssetParams) -> reqwest::Result<GetAssetResponse>
    {
        let request = Req::new(&"getAsset".to_string(), params);
         return parse::<GetAssetResponse>(self.handler.post(self.get_das_url(), &request));
        // return res.map(|r| r.result);
    }

    pub fn get_asset_batch(
        &self,
        params: &GetAssetBatchParams) -> reqwest::Result<Vec<GetAssetResponse>>
    {
        let req = Req::new(&"getAssetBatch".to_string(), params);
        return parse::<Vec<GetAssetResponse>>(self.handler.post(self.get_das_url(), &req));
    }

    pub fn get_asset_proof(
        &self,
        params: &GetAssetProofParams) -> reqwest::Result<GetAssetProofResponse>
    {
        let request = Req::new(&"getAssetProof".to_string(), params);
        return parse::<GetAssetProofResponse>(self.handler.post(self.get_das_url(), &request));
    }

    pub fn get_asset_proof_batch(
        &self,
        params: &GetAssetProofBatchParams) -> reqwest::Result<HashMap<String,GetAssetProofResponse>>
    {
        let req = Req::new(&"getAssetProofBatch".to_string(), params);
        return parse::<HashMap<String, GetAssetProofResponse>>(self.handler.post(self.get_das_url(), &req));
    }

    pub fn get_assets_by_owner(
        &self,
        params: &GetAssetsByOwnerParams) -> reqwest::Result<GetAssetResponseList>
    {
        let request = Req::new(&"getAssetsByOwner".to_string(), params);
        return parse::<GetAssetResponseList>(self.handler.post(self.get_das_url(), &request));
    }

    pub fn get_assets_by_authority(
        &self,
        params: &GetAssetsByAuthorityParams) -> reqwest::Result<GetAssetResponseList>
    {
        let req = Req::new(&"getAssetsByAuthority".to_string(), params);
        return parse::<GetAssetResponseList>(self.handler.post(self.get_das_url(), &req));
    }

    pub fn get_assets_by_creator(
        &self,
        params: &GetAssetsByCreatorParams) -> reqwest::Result<GetAssetResponseList>
    {
        let req = Req::new(&"getAssetsByCreator".to_string(), params);
        return parse::<GetAssetResponseList>(self.handler.post(self.get_das_url(), &req));
    }

    pub fn get_assets_by_group(
        &self,
        params: &GetAssetsByGroupParams) -> reqwest::Result<GetAssetResponseList>
    {
        let req = Req::new(&"getAssetsByGroup".to_string(), params);
        return parse::<GetAssetResponseList>(self.handler.post(self.get_das_url(), &req));
    }

    pub fn search_assets(
        &self,
        params: &SearchAssetsParams) -> reqwest::Result<GetAssetResponseList>
    {
        let req = Req::new(&"searchAssets".to_string(), params);
        return parse::<GetAssetResponseList>(self.handler.post(self.get_das_url(), &req));
    }
}

serializable! {
    struct Req<T> {
        pub jsonrpc: String,
        pub id: String,
        pub method: String,
        pub params: T
    }
}

impl<T> Req<T> {
    pub fn new(method: &String, params: T) -> Req<T> {
        return Req {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.clone(),
            params
        }
    }
}

serializable! {
    pub struct Res<T> {
        pub jsonrpc: String,
        pub id: String,
        pub result: T
    }
}