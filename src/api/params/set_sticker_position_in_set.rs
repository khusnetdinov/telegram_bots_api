use serde::Serialize;

/// https://core.telegram.org/bots/api#setstickerpositioninset
/// Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetStickerPositionInSet {
    sticker: String,
    position: i64,
}
