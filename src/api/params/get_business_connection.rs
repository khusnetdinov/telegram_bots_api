use serde::Serialize;

/// <https://core.telegram.org/bots/api#getbusinessconnection>
/// Use this method to get information about the connection of the bot with a business account. Returns a BusinessConnection object on success.
#[derive(Debug, Serialize, Default)]
pub struct GetBusinessConnection {
    pub business_connection_id: String,
}
