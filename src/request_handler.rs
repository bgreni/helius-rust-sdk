use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;

type Return<K> = reqwest::Result<K>;

pub struct RequestHandler {
    http_client: reqwest::blocking::Client,
}

impl RequestHandler {
    pub fn new() -> RequestHandler {
        return RequestHandler{
            http_client: reqwest::blocking::Client::new()
        };
    }

    pub fn get<U, T>(&self, url: U) -> Return<T> where
        U: IntoUrl,
        T: DeserializeOwned
    {
        return self.http_client
            .get(url)
            .send()?
            .error_for_status()?
            .json();
    }

    pub fn post<R, U, T>(&self, url: U, body: &R) -> Return<T> where
        R: Serialize + ?Sized,
        U: IntoUrl,
        T: DeserializeOwned
    {
        return self.http_client
            .post(url)
            .json(&body)
            .send()?
            .error_for_status()?
            .json();
    }

    pub fn put<R, U, T>(&self, url: U, body: &R) -> Return<T> where
        R: Serialize + ?Sized,
        U: IntoUrl,
        T: DeserializeOwned
    {
        return self.http_client
            .put(url)
            .json(&body)
            .send()?
            .error_for_status()?
            .json();
    }

    pub fn delete<U>(&self, url: U) -> Return<reqwest::blocking::Response> where
        U: IntoUrl
    {
        return self.http_client
            .get(url)
            .send()?
            .error_for_status();
    }
}