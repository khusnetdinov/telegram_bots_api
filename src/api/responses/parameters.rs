use serde::Deserialize;

/// https://core.telegram.org/bots/api#responseparameters
#[derive(Debug, Deserialize, PartialEq)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}
