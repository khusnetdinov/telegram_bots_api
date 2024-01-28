use crate::config::Config;
use std::time::Duration;

#[derive(Debug)]
pub struct Async {
    client: reqwest::Client,
    url: String,
}

impl Async {
    pub fn new(config: &Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .build()
            .unwrap();

        let url = config.build_url();

        Self { client, url }
    }
}
