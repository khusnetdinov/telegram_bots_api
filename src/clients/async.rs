use crate::api::requests::r#async::Requests;
use crate::api::responses::error::ResponseError;
use crate::api::responses::result::ResponseResult;
use crate::api::types::user::User;
use crate::config::Config;
use crate::errors::Error;
use reqwest::Response;
use reqwest::{ClientBuilder, RequestBuilder};
use serde::de::DeserializeOwned;
use std::time::Duration;

#[derive(Debug)]
pub struct Async {
    pub client: reqwest::Client,
    offset: i64,
    limit: i64,
    timeout: u64,
    pub url: String,
}

impl From<&Config> for Async {
    fn from(config: &Config) -> Self {
        let offset = config.updates_offset;
        let limit = config.updates_limit;
        let timeout = config.updates_timeout;
        let url = config.build_url();
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        Self {
            client,
            offset,
            limit,
            timeout,
            url,
        }
    }
}

impl Async {
    pub fn new() -> Self {
        let config = Config::new();
        let offset = config.updates_offset;
        let limit = config.updates_limit;
        let timeout = config.updates_timeout;
        let url = config.build_url();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        Self {
            client,
            offset,
            limit,
            timeout,
            url,
        }
    }

    fn request(&self, method: &str) -> RequestBuilder {
        self.client.post(format!("{}{}", self.url, method))
    }

    async fn respond_with<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, Error> {
        match request.send().await {
            Ok(response) => match response.status().as_u16() {
                200 => self.decode::<T>(response).await,
                400..=429 => Err(Error::Response(ResponseError::from(
                    &response.text().await?,
                ))),
                _ => Err(Error::Unexpected(String::from(""))),
            },
            Err(error) => Err(Error::Request(error)),
        }
    }

    async fn decode<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        match serde_json::from_str::<ResponseResult<T>>(&response.text().await?) {
            Ok(success) => Ok(success.result),
            Err(error) => Err(Error::Decode(error)),
        }
    }
}

impl Requests for Async {
    async fn get_me(&self) -> Result<User, Error> {
        self.respond_with::<User>(self.request("getMe").json(&{}))
            .await
    }
}
