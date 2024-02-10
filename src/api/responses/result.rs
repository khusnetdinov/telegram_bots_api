use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseResult<T> {
    pub result: T,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
