use crate::api::types::input_file::InputFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FileInput {
    InputFile(InputFile),
    String(String),
}
