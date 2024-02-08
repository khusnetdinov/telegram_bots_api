use crate::api::types::chat_id::ChatId;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChatUId {
    I64(ChatId),
    Username(String),
}

impl From<i64> for ChatUId {
    fn from(chat_id: i64) -> Self {
        Self::I64(ChatId::from(chat_id))
    }
}

impl From<&str> for ChatUId {
    fn from(username: &str) -> Self {
        Self::Username(String::from(username))
    }
}

impl Default for ChatUId {
    fn default() -> Self {
        Self::I64(ChatId(0))
    }
}
