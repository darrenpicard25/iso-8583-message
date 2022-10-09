use crate::field::error::FieldError;
use std::fmt::Display;

#[derive(Debug)]
pub enum IsoMessageError {
    InvalidInput(u8),
}

impl Display for IsoMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsoMessageError::InvalidInput(data_element) => write!(
                f,
                "An error occurred packing/unpacking data element field: {data_element}"
            ),
        }
    }
}

impl From<FieldError> for IsoMessageError {
    fn from(error: FieldError) -> Self {
        eprintln!("{error}");

        let data_element = match error {
            FieldError::ByteFieldDecoding(de, _) => de,
            FieldError::StringFieldDecoding(de, _) => de,
            FieldError::FieldLengthDecoding(de, _) => de,
            FieldError::FormatDecoding(de, _) => de,
            FieldError::FormatEncoding(de, _) => de,
        };

        IsoMessageError::InvalidInput(data_element)
    }
}
