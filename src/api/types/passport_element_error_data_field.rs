use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#passportelementerrordatafield
/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorDataField {
    source: String,
    // type: String,
    field_name: String,
    data_hash: String,
    message: String,
}
