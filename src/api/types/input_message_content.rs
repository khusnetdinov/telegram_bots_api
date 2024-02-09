use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputmessagecontent
/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// InputTextMessageContent
/// InputLocationMessageContent
/// InputVenueMessageContent
/// InputContactMessageContent
/// InputInvoiceMessageContent
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputMessageContent {}
