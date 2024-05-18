use crate::api::structs::user::User;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#chatboostsourcepremium>
/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourcePremium {
    pub source: String,
    pub user: User,
}
