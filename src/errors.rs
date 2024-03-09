use crate::api::responses::error::ResponseError;
use std::fmt::{Display, Formatter};

/// Errors for telegram bots api.
#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Response(ResponseError),
    Decode(serde_json::error::Error),
    Unexpected(String),
    Debug,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Request(error) => {
                write!(f, "Request Error: {error}")
            }
            Error::Response(error) => {
                write!(
                    f,
                    "Response Error: error code {}: {}",
                    error.error_code, error.description
                )
            }
            Error::Decode(error) => {
                write!(f, "Decode Error: {}", error.to_string().to_lowercase())
            }
            Error::Unexpected(error) => {
                write!(f, "Unexpected Error: {}", error)
            }
            _ => write!(f, "Debug Error!"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Error:"
    }
}
