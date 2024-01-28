use crate::clients::blocking::Blocking;
use crate::clients::r#async::Async;
use crate::config::Config;

#[derive(Debug)]
pub struct Client {
    pub blocking: Blocking,
    pub r#async: Async,
    pub config: Config,
}

impl Client {
    pub fn new() -> Self {
        let config = Config::new();
        let blocking = Blocking::new(&config);
        let r#async = Async::new(&config);

        Self {
            config,
            r#async,
            blocking,
        }
    }
}
