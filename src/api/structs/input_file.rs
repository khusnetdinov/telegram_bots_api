use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// <https://core.telegram.org/bots/api#inputfile>
/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputFile {
    pub path: PathBuf,
}
