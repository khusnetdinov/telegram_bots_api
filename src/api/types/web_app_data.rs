use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#webappdata
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebAppData {
    data: String,
    button_text: String,
}
