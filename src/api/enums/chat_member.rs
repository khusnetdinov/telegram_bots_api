use crate::api::types::chat_member_administrator::ChatMemberAdministrator;
use crate::api::types::chat_member_banned::ChatMemberBanned;
use crate::api::types::chat_member_left::ChatMemberLeft;
use crate::api::types::chat_member_member::ChatMemberMember;
use crate::api::types::chat_member_owner::ChatMemberOwner;
use crate::api::types::chat_member_restricted::ChatMemberRestricted;
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
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}
