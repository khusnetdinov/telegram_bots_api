use serde::{Deserialize, Serialize};

// https://core.telegram.org/bots/api#generalforumtopichidden
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralForumTopicHidden {}
