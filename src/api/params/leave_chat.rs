use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#leavechat
/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Debug, Serialize)]
pub struct LeaveChat {
    chat_id: ChatUId,
}
