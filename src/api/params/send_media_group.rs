use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::media_input::MediaInput;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendmediagroup
/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
#[derive(Debug, Serialize)]
pub struct SendMediaGroup {
    chat_id: ChatUId,
    message_thread_id: Option<i64>,
    media: Vec<MediaInput>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_parameters: Option<ReplyParameters>,
}
