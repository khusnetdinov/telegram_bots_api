use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboostsourcegiftcode
/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostSourceGiftCode {
    pub source: String,
    pub user: User,
}
