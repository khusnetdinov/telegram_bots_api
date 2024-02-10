use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#reactiontypecustomemoji
/// The reaction is based on a custom emoji.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeCustomEmoji {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub custom_emoji_id: String,
}
