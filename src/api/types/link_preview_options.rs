use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#linkpreviewoptions
/// Describes the options used for link preview generation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>,
}
