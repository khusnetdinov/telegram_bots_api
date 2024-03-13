use crate::api::enums::chat_uid::ChatUId;
use crate::api::structs::message_id::MessageId;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#copymessages>
/// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessages, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of MessageId of the sent messages is returned.
#[derive(Default, Serialize)]
pub struct CopyMessages {
    pub chat_id: ChatUId,
    pub from_chat_id: ChatUId,
    pub message_ids: Vec<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}
