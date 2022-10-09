mod content_type;
pub mod error;
mod len_type;

use serde::Deserialize;

use self::{content_type::ContentType, error::FieldError, len_type::LengthType};

#[derive(Deserialize, Debug)]
pub struct Field {
    pub data_element: u8,
    content_type: ContentType,
    // label: &'static str,
    len_type: LengthType,
    max_len: usize,
    // min_len: Option<usize>,
}

impl Field {
    pub fn value_to_buffer(&self, value: &str) -> Result<Vec<u8>, FieldError> {
        if !self.content_type.is_valid(value) {
            return Err(FieldError::FormatEncoding(
                self.data_element,
                value.to_owned(),
            ));
        }

        let mut value_buf = match self.content_type {
            ContentType::Bytes => self.decode_hex(value)?,
            _ => value.as_bytes().to_vec(),
        };
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

            Ok(buffer)
        } else {
            value_buf.resize(self.max_len, 0);

            Ok(value_buf)
        }
    }

    pub fn get_value_from_buffer(&self, buffer: &[u8]) -> Result<(String, usize), FieldError> {
        let value = if let Some(leading_digits) = self.len_type.get_leading_digits() {
            let var_len = self.extract_field_length(&buffer[..leading_digits])?;
            let value = self.extract_value(&buffer[leading_digits..var_len + leading_digits])?;

            return Ok((value, var_len + leading_digits));
        } else {
            let value = self.extract_value(&buffer[..self.max_len])?;

            Ok((value, self.max_len))
        }?;

        if self.content_type.is_valid(&value.0) {
            return Ok(value);
        }

        Err(FieldError::FormatDecoding(self.data_element, value.0))
    }

    fn extract_field_length(&self, buffer_slice: &[u8]) -> Result<usize, FieldError> {
        String::from_utf8(buffer_slice.to_vec())
            .map_err(|_| FieldError::FieldLengthDecoding(self.data_element, buffer_slice.to_vec()))?
            .parse::<usize>()
            .map_err(|_| FieldError::FieldLengthDecoding(self.data_element, buffer_slice.to_vec()))
    }

    fn extract_value(&self, buffer_slice: &[u8]) -> Result<String, FieldError> {
        let value = match self.content_type {
            ContentType::Bytes => self.encode_hex(buffer_slice),
            _ => String::from_utf8(buffer_slice.to_vec()).map_err(|_| {
                FieldError::StringFieldDecoding(self.data_element, buffer_slice.to_vec())
            })?,
        };

        Ok(value)
    }

    fn encode_hex(&self, bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    fn decode_hex(&self, s: &str) -> Result<Vec<u8>, FieldError> {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&s[i..i + 2], 16)
                    .map_err(|_| FieldError::ByteFieldDecoding(self.data_element, s.to_owned()))
            })
            .collect()
    }
}
