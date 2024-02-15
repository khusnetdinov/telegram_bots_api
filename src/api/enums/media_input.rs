use crate::api::types::input_media_audio::InputMediaAudio;
use crate::api::types::input_media_document::InputMediaDocument;
use crate::api::types::input_media_photo::InputMediaPhoto;
use crate::api::types::input_media_video::InputMediaVideo;
use serde::Serialize;

/// A JSON-serialized array describing messages to be sent, must include 2-10 items
#[derive(Debug, Serialize)]
pub enum MediaInput {
    InputMediaAudio(InputMediaAudio),
    InputMediaDocument(InputMediaDocument),
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}
