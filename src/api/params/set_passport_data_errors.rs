use crate::api::enums::passport_element_error::PassportElementError;
use serde::Serialize;

/// https://core.telegram.org/bots/api#setpassportdataerrors
/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
/// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
#[derive(Debug, Serialize)]
pub struct SetPassportDataErrors {
    pub user_id: i64,
    pub errors: Vec<PassportElementError>,
}
