use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#generalforumtopichidden>
/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicHidden {}
