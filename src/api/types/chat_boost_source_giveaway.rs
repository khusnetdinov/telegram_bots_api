use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatboostsourcegiveaway>
/// The boost was obtained by the creation of a Telegram Premium giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourceGiveaway {
    pub source: String,
    pub giveaway_message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}
