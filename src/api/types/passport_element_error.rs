use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PassportElementError {}
