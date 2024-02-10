use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#file
/// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
/// The maximum file size to download is 20 MB
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_path: String,
}
