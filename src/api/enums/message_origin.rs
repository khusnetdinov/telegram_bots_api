use crate::api::types::message_origin_channel::MessageOriginChannel;
use crate::api::types::message_origin_chat::MessageOriginChat;
use crate::api::types::message_origin_hidden_user::MessageOriginHiddenUser;
use crate::api::types::message_origin_user::MessageOriginUser;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageorigin
/// This object describes the origin of a message. It can be one of
/// MessageOriginUser
/// MessageOriginHiddenUser
/// MessageOriginChat
/// MessageOriginChannel
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MessageOrigin {
    MessageOriginUser(MessageOriginUser),
    MessageOriginHiddenUser(MessageOriginHiddenUser),
    MessageOriginChat(MessageOriginChat),
    MessageOriginChannel(MessageOriginChannel),
}
