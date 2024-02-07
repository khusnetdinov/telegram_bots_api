#[cfg(test)]
mod tests {
    use crate::errors::Error;

    fn error_request() {
        todo!()
    }

    fn response_error() {
        todo!()
    }

    fn decode_error() {
        todo!()
    }

    #[test]
    fn debug_error() {
        let error = Error::Debug;
        let error_string = format!("{}", error);

        assert_eq!(error_string, "Debug Error!");
    }
}
