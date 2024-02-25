use crate::api::enums::file_input::FileInput;
use crate::api::types::mask_position::MaskPosition;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputsticker
/// This object describes a sticker to be added to a sticker set.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InputSticker {
    pub sticker: FileInput,
    pub emoji_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}
