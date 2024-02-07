use crate::api::types::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboost
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoost {
    boost_id: String,
    add_date: i64,
    expiration_date: i64,
    source: ChatBoostSource,
}
