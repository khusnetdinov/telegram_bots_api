use crate::api::requests::Requests;
use crate::api::responses::{ResponseError, ResponseSuccess};
use crate::api::types::User;
use crate::config::Config;
use crate::errors::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct Blocking {
    client: reqwest::blocking::Client,
    url: String,
}

impl Blocking {
    pub fn new(config: &Config) -> Blocking {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let url = config.build_url();

        Blocking { client, url }
    }
}

impl Requests for Blocking {
    type Error = Error;
    type Response<T> = ResponseSuccess<T>;

    fn get_me(&self) -> Result<User, Self::Error> {
        let request = self.client.post(format!("{}{}", self.url, "getMe"));

        match request.send() {
            Ok(response) => match response.status().as_u16() {
                200 => {
                    match serde_json::from_str::<Self::Response<User>>(&response.text().unwrap()) {
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
    }
}
