use crate::api::structs::chat::Chat;
use crate::api::structs::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatboostupdated>
/// This object represents a boost added to a chat or changed.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}
