use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#inputfile
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputFile {}
