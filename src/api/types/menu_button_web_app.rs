use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#menubuttonwebapp
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonWebApp {
    // type: String,
    text: String,
    web_app: WebAppInfo,
}
