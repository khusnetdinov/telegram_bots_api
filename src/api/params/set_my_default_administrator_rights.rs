use crate::api::types::chat_administrator_rights::ChatAdministratorRights;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setmydefaultadministratorrights
/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success.
#[derive(Debug, Serialize, Default)]
pub struct SetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
