use crate::api::enums::chat_uid::ChatUId;
use crate::api::enums::file_input::FileInput;
use crate::api::enums::reply_markup::ReplyMarkup;
use crate::api::types::reply_parameters::ReplyParameters;
use serde::Serialize;

/// https://core.telegram.org/bots/api#sendsticker
/// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
#[derive(Debug, Serialize)]
pub struct SendSticker {
    pub chat_id: ChatUId,
    pub message_thread_id: Option<i64>,
    pub sticker: FileInput,
    pub emoji: Option<String>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_parameters: Option<ReplyParameters>,
    pub reply_markup: Option<ReplyMarkup>,
}
