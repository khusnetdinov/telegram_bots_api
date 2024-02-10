use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultsbutton
/// This object represents a button to be shown above inline query results. You must use exactly one of the optional fields.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultsButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}
