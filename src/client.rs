use crate::clients::r#async::Async;
use crate::clients::sync::Sync;
use crate::config::Config;

#[derive(Debug)]
pub struct Client {
    pub sync: Sync,
    pub r#async: Async,
    pub config: Config,
}

impl Client {
    pub fn new() -> Self {
        let config = Config::new();
        let sync = Sync::new(&config);
        let r#async = Async::new(&config);

        Self {
            config,
            r#async,
            sync,
        }
    }
}
