use serde::Serialize;

/// https://core.telegram.org/bots/api#getmydefaultadministratorrights
/// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
#[derive(Debug, Serialize, Default)]
pub struct GetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
