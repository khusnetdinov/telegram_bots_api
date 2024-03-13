use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::input_media::InputMedia;
use crate::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#editmessagemedia>
/// Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize, Default)]
pub struct EditMessageMedia {
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatUId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
