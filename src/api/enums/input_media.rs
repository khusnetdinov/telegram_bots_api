use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmedia
/// This object represents the content of a media message to be sent. It should be one of
/// InputMediaAnimation
/// InputMediaDocument
/// InputMediaAudio
/// InputMediaPhoto
/// InputMediaVideo
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMedia {}
