use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}