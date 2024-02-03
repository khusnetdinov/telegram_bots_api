use crate::api::types::mask_position::MaskPosition;
use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputsticker
#[derive(Debug, Serialize, Deserialize)]
pub struct InputSticker {
    // sticker: InputFile or String,
    emoji_list: Vec<String>,
    mask_position: Option<MaskPosition>,
    keywords: Option<Vec<String>>,
}
