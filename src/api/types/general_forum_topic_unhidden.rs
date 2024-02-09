use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#generalforumtopicunhidden
/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicUnhidden {}
