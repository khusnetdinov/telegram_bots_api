use crate::api::enums::file_input::FileInput;
use serde::Serialize;

/// <https://core.telegram.org/bots/api#setstickersetthumbnail>
/// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetStickerSetThumbnail {
    pub name: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileInput>,
}
