use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#forumtopicreopened
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicReopened {}
