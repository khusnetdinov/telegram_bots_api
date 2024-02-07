use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#generalforumtopichidden
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicHidden {}
