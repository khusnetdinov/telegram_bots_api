use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inlinequeryresultsbutton
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
    text: String,
    web_app: Option<WebAppInfo>,
    start_parameter: Option<String>,
}
