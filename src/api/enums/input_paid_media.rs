use crate::api::structs::input_paid_media_photo::InputPaidMediaPhoto;
use crate::api::structs::input_paid_media_video::InputPaidMediaVideo;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputpaidmedia>
/// This object describes the paid media to be sent. Currently, it can be one of
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}
