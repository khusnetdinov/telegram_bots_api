use serde::Deserialize;

/// https://core.telegram.org/bots/api#responseparameters
#[derive(Debug, Deserialize, PartialEq)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}
