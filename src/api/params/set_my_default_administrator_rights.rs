use crate::api::types::chat_administrator_rights::ChatAdministratorRights;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setmydefaultadministratorrights
/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success.
#[derive(Debug, Serialize)]
pub struct SetMyDefaultAdministratorRights {
    rights: Option<ChatAdministratorRights>,
    for_channels: Option<bool>,
}
