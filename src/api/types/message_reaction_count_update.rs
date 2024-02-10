use crate::api::types::chat::Chat;
use crate::api::types::message_id::MessageId;
use crate::api::types::reaction_count::ReactionCount;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messagereactioncountupdated
/// This object represents reaction changes on a message with anonymous reactions.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}
