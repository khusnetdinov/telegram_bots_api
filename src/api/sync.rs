use crate::api::Requests;

#[derive(Debug, Clone, Copy)]
pub struct Sync {}

impl Sync {
    pub fn new() -> Self {
        Sync {}
    }
}

impl Requests for Sync {}
