use crate::api::types::input_file::InputFile;
use serde::Serialize;

/// https://core.telegram.org/bots/api#uploadstickerfile
/// Use this method to upload a file with a sticker for later use in the createNewStickerSet and addStickerToSet methods (the file can be used multiple times). Returns the uploaded File on success.
#[derive(Debug, Serialize)]
pub struct UploadStickerFile {
    user_id: i64,
    sticker: InputFile,
    sticker_format: String,
}
