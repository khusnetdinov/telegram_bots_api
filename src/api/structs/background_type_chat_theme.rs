use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#backgroundtypefill>
/// The background is automatically filled based on the selected colors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeChatTheme {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub theme_name: String,
}
