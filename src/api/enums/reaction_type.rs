use crate::api::structs::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::api::structs::reaction_type_emoji::ReactionTypeEmoji;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#reactiontype>
/// This object describes the type of a reaction. Currently, it can be one of
/// ReactionTypeEmoji
/// ReactionTypeCustomEmoji
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
}
