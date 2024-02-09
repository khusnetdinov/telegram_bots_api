use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#reactiontype
/// This object describes the type of a reaction. Currently, it can be one of
/// ReactionTypeEmoji
/// ReactionTypeCustomEmoji
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ReactionType {}
