use crate::api::enums::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatboost>
/// This object contains information about a chat boost.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: ChatBoostSource,
}
