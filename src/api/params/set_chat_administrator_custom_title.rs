use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setchatadministratorcustomtitle
/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    pub chat_id: ChatUId,
    pub user_id: i64,
    pub custom_title: String,
}
