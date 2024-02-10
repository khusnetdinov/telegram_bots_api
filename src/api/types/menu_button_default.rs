use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#menubuttondefault
/// Describes that no specific value for the menu button was set.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonDefault {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
