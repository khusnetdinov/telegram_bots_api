use crate::api::structs::passport_element_error_data_field::PassportElementErrorDataField;
use crate::api::structs::passport_element_error_file::PassportElementErrorFile;
use crate::api::structs::passport_element_error_files::PassportElementErrorFiles;
use crate::api::structs::passport_element_error_front_side::PassportElementErrorFrontSide;
use crate::api::structs::passport_element_error_reverse_side::PassportElementErrorReverseSide;
use crate::api::structs::passport_element_error_selfie::PassportElementErrorSelfie;
use crate::api::structs::passport_element_error_translation_file::PassportElementErrorTranslationFile;
use crate::api::structs::passport_element_error_translation_files::PassportElementErrorTranslationFiles;
use crate::api::structs::passport_element_error_unspecified::PassportElementErrorUnspecified;
use serde::{Deserialize, Serialize};

/// <https://core.telegram.org/bots/api#passportelementerror>
/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// PassportElementErrorDataField
/// PassportElementErrorFrontSide
/// PassportElementErrorReverseSide
/// PassportElementErrorSelfie
/// PassportElementErrorFile
/// PassportElementErrorFiles
/// PassportElementErrorTranslationFile
/// PassportElementErrorTranslationFiles
/// PassportElementErrorUnspecified
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PassportElementError {
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
    Unspecified(PassportElementErrorUnspecified),
}
