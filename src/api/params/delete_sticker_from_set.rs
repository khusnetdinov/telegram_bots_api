use serde::Serialize;

/// <https://core.telegram.org/bots/api#deletestickerfromset>
/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct DeleteStickerFromSet {
    pub sticker: String,
}
