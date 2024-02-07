use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#webappinfo
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebAppInfo {
    url: String,
}
