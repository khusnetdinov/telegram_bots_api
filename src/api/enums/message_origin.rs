use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageorigin
/// This object describes the origin of a message. It can be one of
/// MessageOriginUser
/// MessageOriginHiddenUser
/// MessageOriginChat
/// MessageOriginChannel
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOrigin {}
