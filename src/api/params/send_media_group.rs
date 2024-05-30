use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::input_media::InputMedia;
use crate::api::structs::reply_parameters::ReplyParameters;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#sendmediagroup>
/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
#[derive(Debug, Serialize, Default)]
pub struct SendMediaGroup {
    pub chat_id: ChatUId,
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
}
