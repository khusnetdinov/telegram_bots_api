use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseSuccess<T> {
    ok: bool,
    description: Option<String>,
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    ok: bool,
    error_code: u64,
    description: String,
    parameters: Option<ResponseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}
