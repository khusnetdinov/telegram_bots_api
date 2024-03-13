use crate::api::structs::input_contact_message_content::InputContactMessageContent;
use crate::api::structs::input_invoice_message_content::InputInvoiceMessageContent;
use crate::api::structs::input_location_message_content::InputLocationMessageContent;
use crate::api::structs::input_text_message_content::InputTextMessageContent;
use crate::api::structs::input_venue_message_content::InputVenueMessageContent;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#inputmessagecontent>
/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 structs:
/// InputTextMessageContent
/// InputLocationMessageContent
/// InputVenueMessageContent
/// InputContactMessageContent
/// InputInvoiceMessageContent
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
    InputInvoiceMessageContent(InputInvoiceMessageContent),
}

impl Default for InputMessageContent {
    fn default() -> Self {
        Self::InputTextMessageContent(InputTextMessageContent {
            ..Default::default()
        })
    }
}
