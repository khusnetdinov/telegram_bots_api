use serde::Serialize;

/// https://core.telegram.org/bots/api#getstickerset
/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Debug, Serialize)]
pub struct GetStickerSet {
    name: String,
}
