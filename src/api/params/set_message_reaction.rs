use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reaction_type::ReactionType;
use crate::api::types::message_id::MessageId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setmessagereaction
/// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetMessageReaction {
    chat_id: ChatUId,
    message_id: MessageId,
    reaction: Option<Vec<ReactionType>>,
    is_big: Option<bool>,
}
