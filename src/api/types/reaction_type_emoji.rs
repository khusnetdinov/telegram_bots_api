use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#reactiontypeemoji>
/// The reaction is based on an emoji.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeEmoji {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub emoji: String,
}
