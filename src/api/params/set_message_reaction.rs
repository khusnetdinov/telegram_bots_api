use crate::api::types::reaction_type::ReactionType;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setmessagereaction
/// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetMessageReaction {
    chat_id: i64,
    message_id: i64,
    reaction: Option<Vec<ReactionType>>,
    is_big: Option<bool>,
}
