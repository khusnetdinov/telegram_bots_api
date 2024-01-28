use crate::api::responses::ResponseError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Response(ResponseError),
    Decode(serde_json::error::Error),
    Debug,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Request(error) => write!(f, "{:#?}", error),
            Error::Response(error) => write!(f, "{:#?}", error),
            Error::Decode(error) => write!(f, "{:#?}", error),
            _ => write!(f, "Debug"),
        }
    }
}
