use crate::errors::Error;
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;

pub trait Encoder {
    fn encode<P>(params: &P) -> Result<String, Error>;
}

pub trait Decoder {
    fn decode<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error>;
}

pub trait Responder: Decoder {
    fn respond_with<T: DeserializeOwned>(
        &self,
        response: Result<Response, reqwest::Error>,
    ) -> Result<T, Error>;
}
