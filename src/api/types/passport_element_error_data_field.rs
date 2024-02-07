use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerrordatafield
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorDataField {
    source: String,
    // type: String,
    field_name: String,
    data_hash: String,
    message: String,
}
