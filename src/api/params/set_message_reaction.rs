use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reaction_type::ReactionType;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#setmessagereaction>
/// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetMessageReaction {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}
