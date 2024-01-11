use crate::config::Config;
use hyper::Uri;
use std::error::Error;

#[derive(Debug)]
pub struct Api {
    pub address: String,
    pub config: Config,
    pub uri: Uri,
}

impl Api {
    pub fn new() -> Result<Api, Box<dyn Error>> {
        let config = Config::new();
        let url = format!("{}bot{}/", config.url, config.token);
        let uri = url.parse::<hyper::Uri>().unwrap();
        let host = uri.host().unwrap();
        let port = uri.port_u16().unwrap_or(80);
        let address = format!("{}:{}", host, port);

        Ok(Api {
            address,
            config,
            uri,
        })
    }
}
