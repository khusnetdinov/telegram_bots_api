use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#forumtopicclosed
#[derive(Debug, Serialize, Deserialize)]
pub struct ForumTopicClosed {}
