use crate::config::Config;
use std::time::Duration;

#[derive(Debug)]
pub struct Async {
    client: reqwest::Client,
    offset: i64,
    limit: i64,
    timeout: u64,
    url: String,
}

impl Async {
    pub fn new(config: &Config) -> Self {
        let client = reqwest::Client::builder()
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
