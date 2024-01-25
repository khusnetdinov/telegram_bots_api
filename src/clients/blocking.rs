use crate::api::requests::Requests;
use crate::api::responses::{ResponseError, ResponseSuccess};
use crate::api::types::User;
use crate::config::Config;
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
    type Error = ResponseError;
    type Success<T> = ResponseSuccess<T>;

    fn get_me(&self) -> Result<Self::Success<User>, Self::Error> {
        let response = self
            .client
            .post(format!("{}{}", self.url, "getMe"))
            .send()
            .unwrap();
        let body = response.text().unwrap();

        match serde_json::from_str::<Self::Success<User>>(&body) {
            Ok(success) => Ok(success),
            Err(_) => Err(Self::Error::new(&body))
        }
    }
}
