use crate::api::responses::ResponseError;

impl ResponseError {
    pub fn new(body: &str) -> Self {
        serde_json::from_str::<ResponseError>(body).unwrap()
    }
}
