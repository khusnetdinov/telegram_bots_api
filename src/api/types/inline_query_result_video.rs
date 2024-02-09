use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#inlinequeryresultvideo
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InlineQueryResultVideo {}
