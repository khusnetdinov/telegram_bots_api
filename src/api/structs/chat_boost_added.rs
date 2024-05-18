use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatboostadded>
/// This object represents a service message about a user boosting a chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostAdded {
    pub boost_count: u64,
}
