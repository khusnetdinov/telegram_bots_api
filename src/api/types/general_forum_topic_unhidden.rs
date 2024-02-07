use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#generalforumtopicunhidden
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicUnhidden {}
