use serde::Serialize;

/// https://core.telegram.org/bots/api#getuserchatboosts
/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a UserChatBoosts object.
#[derive(Debug, Serialize)]
pub struct GetUserChatBoosts {
    chat_id: i64,
    user_id: i64,
}
