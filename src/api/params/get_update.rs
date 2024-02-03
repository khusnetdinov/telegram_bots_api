use serde::Serialize;

/// https://core.telegram.org/bots/api#getupdates
#[derive(Debug, Serialize)]
pub struct GetUpdate {
    pub offset: i64,
    pub limit: i64,
    pub timeout: u64,
}
