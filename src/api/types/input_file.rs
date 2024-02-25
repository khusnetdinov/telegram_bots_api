use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputfile
/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct InputFile {}
