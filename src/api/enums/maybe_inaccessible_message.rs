use crate::api::types::inaccessible_message::InaccessibleMessage;
use crate::api::types::message::Message;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#maybeinaccessiblemessage
/// This object describes a message that can be inaccessible to the bot. It can be one of
/// Message
/// InaccessibleMessage
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MaybeInaccessibleMessage {
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}
