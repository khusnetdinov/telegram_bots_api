use crate::api::structs::input_file::InputFile;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FileInput {
    InputFile(InputFile),
    String(String),
}

impl Default for FileInput {
    fn default() -> Self {
        Self::String(String::from(""))
    }
}

impl From<PathBuf> for FileInput {
    fn from(path: PathBuf) -> Self {
        let input_file = InputFile { path };

        Self::InputFile(input_file)
    }
}

impl From<InputFile> for FileInput {
    fn from(file: InputFile) -> Self {
        Self::InputFile(file)
    }
}

impl From<String> for FileInput {
    fn from(file: String) -> Self {
        Self::String(file)
    }
}
