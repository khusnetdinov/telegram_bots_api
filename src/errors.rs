use crate::api::responses::ResponseError;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Response(ResponseError),
    Decode(serde_json::error::Error),
    Debug,
}

impl ResponseError {
    pub fn new(body: &str) -> Self {
        serde_json::from_str::<ResponseError>(body).unwrap()
    }
}
