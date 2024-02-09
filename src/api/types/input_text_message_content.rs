use crate::api::types::link_preview_options::LinkPreviewOptions;
use crate::api::types::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inputtextmessagecontent
/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    link_preview_options: Option<LinkPreviewOptions>,
}
