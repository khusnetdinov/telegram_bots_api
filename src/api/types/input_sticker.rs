use crate::api::types::mask_position::MaskPosition;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputsticker
/// This object describes a sticker to be added to a sticker set.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputSticker {
    // sticker: InputFile or String,
    emoji_list: Vec<String>,
    mask_position: Option<MaskPosition>,
    keywords: Option<Vec<String>>,
}
