use serde::Serialize;

/// https://core.telegram.org/bots/api#getmydefaultadministratorrights
/// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
#[derive(Debug, Serialize)]
pub struct GetMyDefaultAdministratorRights {
    pub for_channels: Option<bool>,
}
