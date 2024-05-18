use crate::api::structs::chat::Chat;
use crate::api::structs::message_id::MessageId;
use crate::api::structs::reaction_count::ReactionCount;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messagereactioncountupdated>
/// This object represents reaction changes on a message with anonymous reactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}
