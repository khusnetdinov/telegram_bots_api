use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#getuserchatboosts>
/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a UserChatBoosts object.
#[derive(Debug, Serialize, Default)]
pub struct GetUserChatBoosts {
    pub chat_id: ChatUId,
    pub user_id: i64,
}
