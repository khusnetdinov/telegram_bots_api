use crate::config::Config;
use std::time::Duration;

#[derive(Debug)]
pub struct Reqwest {
    client: reqwest::Client,
    url: String,
}

impl Reqwest {
    pub fn new(config: &Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let url = config.build_url();

        Reqwest { client, url }
    }
}
