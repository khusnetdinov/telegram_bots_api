use crate::api::types::message::Message;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#updating-messages
/// The following methods allow you to change an existing message in the message history instead of sending a new one with a result of an action. This is most useful for messages with inline keyboards using callback queries, but can also help reduce clutter in conversations with regular chat bots.
/// Please note, that it is currently only possible to edit messages without reply_markup or with inline keyboards.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageResult {
    Inline(String),
    Message(Message),
}
