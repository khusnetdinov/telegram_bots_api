use crate::api::structs::message_origin_channel::MessageOriginChannel;
use crate::api::structs::message_origin_chat::MessageOriginChat;
use crate::api::structs::message_origin_hidden_user::MessageOriginHiddenUser;
use crate::api::structs::message_origin_user::MessageOriginUser;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#messageorigin>
/// This object describes the origin of a message. It can be one of
/// MessageOriginUser
/// MessageOriginHiddenUser
/// MessageOriginChat
/// MessageOriginChannel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageOrigin {
    User(MessageOriginUser),
    HiddenUser(MessageOriginHiddenUser),
    Chat(MessageOriginChat),
    Channel(MessageOriginChannel),
}
