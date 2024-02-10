use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatId(pub i64);

impl From<i64> for ChatId {
    fn from(chat_id: i64) -> Self {
        Self(chat_id)
    }
}
