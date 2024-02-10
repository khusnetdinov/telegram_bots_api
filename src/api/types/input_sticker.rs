use crate::api::enums::file_input::FileInput;
use crate::api::types::mask_position::MaskPosition;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputsticker
/// This object describes a sticker to be added to a sticker set.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputSticker {
    pub sticker: FileInput,
    pub emoji_list: Vec<String>,
    pub mask_position: Option<MaskPosition>,
    pub keywords: Option<Vec<String>>,
}
