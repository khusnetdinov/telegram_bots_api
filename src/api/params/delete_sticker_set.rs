use serde::Serialize;

/// https://core.telegram.org/bots/api#deletestickerset
/// Use this method to delete a sticker set that was created by the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct DeleteStickerSet {
    name: String,
}
