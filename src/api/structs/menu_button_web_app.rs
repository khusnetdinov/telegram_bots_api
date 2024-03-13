use crate::api::structs::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#menubuttonwebapp>
/// Represents a menu button, which launches a Web App.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonWebApp {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub text: String,
    pub web_app: WebAppInfo,
}
