use crate::api::structs::input_media_animation::InputMediaAnimation;
use crate::api::structs::input_media_audio::InputMediaAudio;
use crate::api::structs::input_media_document::InputMediaDocument;
use crate::api::structs::input_media_photo::InputMediaPhoto;
use crate::api::structs::input_media_video::InputMediaVideo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputmedia>
/// This object represents the content of a media message to be sent. It should be one of
/// InputMediaAnimation
/// InputMediaDocument
/// InputMediaAudio
/// InputMediaPhoto
/// InputMediaVideo
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl Default for InputMedia {
    fn default() -> Self {
        Self::Animation(InputMediaAnimation {
            ..Default::default()
        })
    }
}
