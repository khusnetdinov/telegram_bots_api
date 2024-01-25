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
