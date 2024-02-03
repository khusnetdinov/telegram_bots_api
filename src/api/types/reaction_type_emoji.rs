use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#reactiontypeemoji
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionTypeEmoji {
    // type: String,
    emoji: String,
}
