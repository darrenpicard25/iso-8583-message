use crate::IsoMessageError;

use super::{content_type::ContentType, length_type::LengthType};
use serde::Deserialize;

fn bitmap_default() -> bool {
    false
}

#[derive(Deserialize, Debug)]
pub struct Field {
    content_type: ContentType,
    label: &'static str,
    len_type: LengthType,
    max_len: usize,
    min_len: Option<usize>,
    #[serde(default = "bitmap_default")]
    is_bitmap: bool,
}

impl Field {
    pub fn is_bitmap(&self) -> bool {
        self.is_bitmap
    }

    pub fn value_to_buffer(&self, value: &str) -> Vec<u8> {
        let mut value_buf = value.as_bytes().to_vec();
        if let Some(leading_digit) = self.len_type.get_leading_digits() {
            let mut buffer = vec![0x30; leading_digit];
            let value_buf_len = value_buf.len().to_string();
            let value_buf_len = value_buf_len.as_bytes();

            buffer.extend_from_slice(value_buf_len);

            let position = if value_buf_len.len() > leading_digit {
                value_buf_len.len() - leading_digit
            } else {
                leading_digit
            };
            let mut buffer = buffer[buffer.len() - position..].to_vec();

            buffer.append(value_buf.as_mut());

            buffer
        } else {
            value_buf.resize(self.max_len, 0);

            value_buf
        }
    }

    pub fn get_value_from_buffer(&self, buffer: &[u8]) -> Result<(String, usize), IsoMessageError> {
        if let Some(leading_digits) = self.len_type.get_leading_digits() {
            let var_len = String::from_utf8(buffer[..leading_digits].to_vec())
                .map_err(|_| {
                    IsoMessageError::InvalidInput(format!(
                        "Unable to get leading digits from field {} as utf-8 from buffer",
                        self.label
                    ))
                })?
                .parse::<usize>()
                .map_err(|_| {
                    IsoMessageError::InvalidInput(format!(
                        "Unable to to convert leading digits for field {} to usize",
                        self.label
                    ))
                })?;

            return Ok((
                String::from_utf8(buffer[leading_digits..var_len + leading_digits].to_vec())
                    .map_err(|_| {
                        IsoMessageError::InvalidInput(format!(
                            "Unable to convert field {} as utf-8 from buffer",
                            self.label
                        ))
                    })?,
                var_len + leading_digits,
            ));
        };
        Ok((
            String::from_utf8(buffer[..self.max_len].to_vec()).map_err(|_| {
                IsoMessageError::InvalidInput(format!(
                    "Unable to convert field {} as utf-8 from buffer",
                    self.label
                ))
            })?,
            self.max_len,
        ))
    }
}
