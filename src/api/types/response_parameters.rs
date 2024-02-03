use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#responseparameters
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}
