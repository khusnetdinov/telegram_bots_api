use crate::api::structs::chat::Chat;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#story>
/// This object represents a message about a forwarded story in the chat. Currently holds no information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    pub chat: Box<Chat>,
    pub id: i64,
}
