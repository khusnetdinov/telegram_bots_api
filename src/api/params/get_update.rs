use serde::Serialize;

/// https://core.telegram.org/bots/api#getupdates
/// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
#[derive(Debug, Serialize, Default)]
pub struct GetUpdate {
    pub offset: i64,
    pub limit: i64,
    pub timeout: u64,
    pub allowed_updates: Option<Vec<String>>,
}
