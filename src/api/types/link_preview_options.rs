use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#linkpreviewoptions
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LinkPreviewOptions {
    is_disabled: Option<bool>,
    url: Option<String>,
    prefer_small_media: Option<bool>,
    prefer_large_media: Option<bool>,
    show_above_text: Option<bool>,
}
