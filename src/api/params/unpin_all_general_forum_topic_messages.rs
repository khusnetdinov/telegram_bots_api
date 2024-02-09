use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
#[derive(Debug, Serialize)]
pub struct UnpinAllGeneralForumTopicMessages {
    chat_id: ChatUId,
}
