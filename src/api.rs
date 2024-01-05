use crate::config::Config;

#[derive(Debug)]
pub struct Api {
    pub config: Config
}

impl Api {
    pub fn new() -> Self {
        Api{
            config: Config::new()
        }
    }

    pub fn observe_polling_updates() {
        todo!();
    }

    pub fn listen_webhook_updates() {
        todo!();
    }
}
