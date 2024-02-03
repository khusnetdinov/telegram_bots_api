use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#reactiontypecustomemoji
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmoji {
    // type: String,
    custom_emoji_id: String,
}
