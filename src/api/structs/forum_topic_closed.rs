use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#forumtopicclosed>
/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicClosed {}
