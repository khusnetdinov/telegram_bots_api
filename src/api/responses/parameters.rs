use serde::Deserialize;

/// <https://core.telegram.org/bots/api#responseparameters>
#[derive(Debug, Deserialize, PartialEq)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}
