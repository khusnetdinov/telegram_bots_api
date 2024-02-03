use super::parameters::ResponseParameters;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    ok: bool,
    error_code: u64,
    description: String,
    parameters: Option<ResponseParameters>,
}

impl ResponseError {
    pub fn new(body: &str) -> Self {
        serde_json::from_str::<ResponseError>(body).unwrap()
    }
}
