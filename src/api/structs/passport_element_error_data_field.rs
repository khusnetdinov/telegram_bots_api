use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#passportelementerrordatafield>
/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorDataField {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String,
}
