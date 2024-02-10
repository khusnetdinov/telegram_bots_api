use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboostsourcegiveaway
/// The boost was obtained by the creation of a Telegram Premium giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourceGiveaway {
    pub source: String,
    pub giveaway_message_id: i64,
    pub user: Option<User>,
    pub is_unclaimed: Option<bool>,
}
