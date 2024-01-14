use crate::config::{Builder, Config};

#[derive(Debug)]
pub struct Api {
    pub client: reqwest::Client,
    pub config: Config,
    pub url: String,
}

impl Api {
    pub fn new() -> Self {
        let config = Config::new();
        let client = config.build_client();
        let url = config.build_url();

        Api {
            config,
            client,
            url,
        }
    }
}
