pub mod errors;
pub mod params;
pub mod requests;
pub mod responses;
pub mod types;

use crate::clients::blocking::Blocking;
use crate::clients::reqwest::Reqwest;
use crate::config::Config;

#[derive(Debug)]
pub struct Api {
    pub blocking: Blocking,
    pub client: Reqwest,
    pub config: Config,
}

impl Api {
    pub fn new() -> Self {
        let config = Config::new();
        let blocking = Blocking::new(&config);
        let client = Reqwest::new(&config);

        Api {
            config,
            client,
            blocking,
        }
    }
}
