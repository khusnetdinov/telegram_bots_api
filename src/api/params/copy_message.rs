use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::structs::message_entity::MessageEntity;
use crate::api::structs::message_id::MessageId;
use crate::api::structs::reply_parameters::ReplyParameters;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#copymessage>
/// Use this method to copy messages of any kind. Service messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
#[derive(Default, Serialize)]
pub struct CopyMessage {
    #[serde(flatten)]
    pub message_id: MessageId,
    pub chat_id: ChatUId,
    pub from_chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}
