use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#reactiontypeemoji
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypeEmoji {
    // type: String,
    emoji: String,
}
