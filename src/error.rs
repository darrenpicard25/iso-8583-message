use std::fmt::Display;

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
