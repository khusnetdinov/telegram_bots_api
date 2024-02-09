use serde::Serialize;

/// https://core.telegram.org/bots/api#getupdates
/// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
#[derive(Debug, Serialize)]
pub struct GetUpdate {
    pub offset: i64,
    pub limit: i64,
    pub timeout: u64,
}
