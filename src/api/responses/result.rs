use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseResult<T> {
    ok: bool,
    description: Option<String>,
    pub result: T,
}
