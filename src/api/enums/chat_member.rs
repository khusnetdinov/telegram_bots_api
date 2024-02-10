use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatmember
/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// ChatMemberOwner
/// ChatMemberAdministrator
/// ChatMemberMember
/// ChatMemberRestricted
/// ChatMemberLeft
/// ChatMemberBanned
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatMember {}
