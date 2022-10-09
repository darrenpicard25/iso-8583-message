use std::fmt::Display;

pub enum FieldError {
    ByteFieldDecoding(u8, String),
    StringFieldDecoding(u8, Vec<u8>),
    FieldLengthDecoding(u8, Vec<u8>),
    FormatDecoding(u8, String),
    FormatEncoding(u8, String),
}

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::FormatEncoding(de, value) => format!("DE:{de} did not match the formatting expected for this field when converting to buffer \n{value}"),
            Self::FormatDecoding(de, value) => format!("DE:{de} did not match the formatting expected for this field when extracting from buffer \n{value}"),
            Self::FieldLengthDecoding(de, value ) => format!("Unable to extract field length from buffer for DE:{de}. \n{:?}", value),
            Self::StringFieldDecoding(de, value ) => format!("Unable to extract field value from buffer for DE:{de} with length: {}, \n{:?}", value.len(), value),
            Self::ByteFieldDecoding(de,value ) => format!("Unable to convert hex string to bytes for DE:{de}.\nValue: {value}")
        };
        write!(f, "{message}")
    }
}
