use crate::api::response::ResponseError;

enum Error {
    Response(ResponseError),
}
