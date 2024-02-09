use crate::api::types::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultsbutton
/// This object represents a button to be shown above inline query results. You must use exactly one of the optional fields.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultsButton {
    text: String,
    web_app: Option<WebAppInfo>,
    start_parameter: Option<String>,
}
