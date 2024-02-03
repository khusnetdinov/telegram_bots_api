use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct ChatId(pub i64);
