use crate::errors::Error;
use reqwest::Response;

pub trait Decode {
    fn decode<R>(response: &Response) -> Result<R, Error>;
}

pub trait Encode {
    fn encode<P>(params: &P) -> Result<String, Error>;
}

pub trait Responder: Decode {
    fn respond_with<T>(response: &Response) -> Result<T, Error>;
}
