use crate::api::params::UpdateParams;
use crate::api::requests::sync::Requests;
use crate::api::responses::ResponseError;
use crate::api::responses::ResponseResult;
use crate::api::types::{Update, User};
use crate::config::Config;
use crate::errors::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct Sync {
    client: reqwest::blocking::Client,
    offset: i64,
    limit: i64,
    timeout: u64,
    url: String,
}

impl Sync {
    pub fn new(config: &Config) -> Self {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let offset = config.updates_offset;
        let limit = config.updates_limit;
        let timeout = config.updates_timeout;
        let url = config.build_url();

        Self {
            client,
            offset,
            limit,
            timeout,
            url,
        }
    }
}

impl Requests for Sync {
    fn get_updates(&self, params: &UpdateParams) -> Result<Vec<Update>, Error> {
        let request = self
            .client
            .post(format!("{}{}", self.url, "getUpdates"))
            .query(params);

        // TODO:begin respond_with<T>(request.send())
        match request.send() {
            Ok(response) => match response.status().as_u16() {
                200 => {
                    match serde_json::from_str::<ResponseResult<Vec<Update>>>(
                        &response.text().unwrap(),
                    ) {
                        Ok(success) => Ok(success.result),
                        Err(error) => Err(Error::Decode(error)),
                    }
                }
                _ => Err(Error::Response(ResponseError::new(
                    &response.text().unwrap(),
                ))),
            },
            Err(error) => {
                println!("Error: {:#?}", error);

                Err(Error::Request(error))
            }
        }
        // TODO:end
    }

    fn get_me(&self) -> Result<User, Error> {
        let request = self.client.post(format!("{}{}", self.url, "getMe"));

        // TODO:begin respond_with<T>(request.send())
        match request.send() {
            Ok(response) => match response.status().as_u16() {
                200 => {
                    match serde_json::from_str::<ResponseResult<User>>(&response.text().unwrap()) {
                        Ok(success) => Ok(success.result),
                        Err(error) => Err(Error::Decode(error)),
                    }
                }
                _ => Err(Error::Response(ResponseError::new(
                    &response.text().unwrap(),
                ))),
            },
            Err(error) => Err(Error::Request(error)),
        }
        // TODO:end
    }
}
