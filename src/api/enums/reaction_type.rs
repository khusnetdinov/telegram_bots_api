use crate::api::structs::reaction_type_custom_emoji::ReactionTypeCustomEmoji;
use crate::api::structs::reaction_type_emoji::ReactionTypeEmoji;
use crate::api::structs::reaction_type_paid::ReactionTypePaid;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#reactiontype>
/// This object describes the type of a reaction. Currently, it can be one of
/// ReactionTypeEmoji
/// ReactionTypeCustomEmoji
/// ReactionTypePaid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
    Paid(ReactionTypePaid),
}
