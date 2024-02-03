use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#chatboostsourcegiveaway
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    source: String,
    giveaway_message_id: i64,
    user: Option<User>,
    is_unclaimed: Option<bool>,
}
