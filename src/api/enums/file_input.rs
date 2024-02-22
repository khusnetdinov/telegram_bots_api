use crate::api::types::input_file::InputFile;
use serde::{Deserialize, Serialize};

/// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FileInput {
    InputFile(InputFile),
    String(String),
}

impl Default for FileInput {
    fn default() -> Self {
        Self::String(String::from(""))
    }
}
