use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#webappinfo
#[derive(Debug, Serialize, Deserialize)]
pub struct WebAppInfo {
    url: String,
}
