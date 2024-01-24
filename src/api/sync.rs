use crate::api::request::Request;
use crate::api::response::{ResponseError, ResponseSuccess};
use crate::api::types::User;

#[derive(Debug, Clone, Copy)]
pub struct Sync {}

impl Sync {
    pub fn new() -> Self {
        Sync {}
    }
}

impl Request for Sync {
    fn get_me(&self) -> Result<ResponseSuccess<User>, ResponseError> {
        todo!()
    }
}
