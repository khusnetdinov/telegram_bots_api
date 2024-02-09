use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#forumtopicreopened
/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicReopened {}
