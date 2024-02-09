use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#reactiontypecustomemoji
/// The reaction is based on a custom emoji.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeCustomEmoji {
    // type: String,
    custom_emoji_id: String,
}
