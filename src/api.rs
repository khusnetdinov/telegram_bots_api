mod requests;
mod sync;

use crate::api::requests::Requests;
use crate::api::sync::Sync;
use crate::config::{Builder, Config};

#[derive(Debug)]
pub struct Api {
    pub client: reqwest::Client,
    pub config: Config,
    pub sync: Sync,
    pub url: String,
}
impl Api {
    pub fn new() -> Self {
        let config = Config::new();
        let client = config.build_client();
        let sync = Sync::new();
        let url = config.build_url();

        Api {
            config,
            client,
            sync,
            url,
        }
    }
}

impl Requests for Api {}
