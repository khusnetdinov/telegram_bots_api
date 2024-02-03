use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#passportelementerror
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportElementError {}
