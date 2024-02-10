use crate::api::enums::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboost
/// This object contains information about a chat boost.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoost {
    boost_id: String,
    add_date: i64,
    expiration_date: i64,
    source: ChatBoostSource,
}
