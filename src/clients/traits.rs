use reqwest::Response;

pub trait Responder {
    type Error;
    fn respond_with<T>(response: &Response) -> Result<T, Self::Error>;
}
pub trait Decode {
    type Error;

    fn decode<T>(response: &Response) -> Result<T, Self::Error>;
}

pub trait Encode {
    type Error;

    fn encode<T>(params: &T) -> Result<String, Self::Error>;
}
