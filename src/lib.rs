use base_format::BASE_ISO_FORMAT;
use message_types::{IsoMessageType, ISO_MESSAGE_TYPE_MAP};
use std::collections::HashMap;

mod base_format;
mod fields;
mod message_types;

pub type IsoJsonMessage = HashMap<&'static str, String>;

#[derive(Debug)]
pub enum IsoMessageError {
    InvalidInput(String),
}

pub struct IsoMessage {
    pub message: IsoJsonMessage,
    pub original_buffer: Option<Vec<u8>>,
}

impl IsoMessage {
    pub fn from_json_message(message: IsoJsonMessage) -> Self {
        IsoMessage {
            message,
            original_buffer: None,
        }
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<Self, IsoMessageError> {
        let message = Self::parse_buffer(&buffer)?;
        Ok(IsoMessage {
            message,
            original_buffer: Some(buffer),
        })
    }

    fn get_bitmaps_from_buffer(buffer: &[u8]) -> Result<(Vec<u64>, usize), IsoMessageError> {
        let mut total_bytes = 0;
        let mut bitmaps = Vec::new();

        let get_bitmap = move |map_name: &str, cursor: usize| -> Result<u64, IsoMessageError> {
            Ok(u64::from_be_bytes(
                buffer[cursor..cursor + 8].try_into().map_err(|_| {
                    IsoMessageError::InvalidInput(format!("Unable to get {map_name} bitmap"))
                })?,
            ))
        };
        let primary_bitmap = get_bitmap("primary", total_bytes)?;
        bitmaps.push(primary_bitmap);
        total_bytes += 8;

        if Self::is_field_in_bitmap(primary_bitmap, 64 - 1) {
            let secondary_bitmap = get_bitmap("secondary", total_bytes)?;
            bitmaps.push(secondary_bitmap);
            total_bytes += 8;

            if Self::is_field_in_bitmap(secondary_bitmap, 64 - 1) {
                let tertiary_bitmap = get_bitmap("tertiary", total_bytes)?;
                bitmaps.push(tertiary_bitmap);
                total_bytes += 8;
            }
        }

        Ok((bitmaps, total_bytes))
    }

    fn is_field_in_bitmap(num: u64, pos: u8) -> bool {
        num & (1 << pos) != 0
    }

    fn parse_buffer(buffer: &Vec<u8>) -> Result<IsoJsonMessage, IsoMessageError> {
        let mut cursor: usize = 0;
        let mut message: IsoJsonMessage = HashMap::new();
        cursor += 2;

        let (mti, parsed_bytes) = BASE_ISO_FORMAT
            .get("0")
            .ok_or(IsoMessageError::InvalidInput(
                "Unable to find field 0 in format".to_owned(),
            ))?
            .get_value_from_buffer(&buffer[cursor..])?;

        cursor += parsed_bytes;
        message.insert("0", mti);

        let (bitmaps, parsed_bytes) = IsoMessage::get_bitmaps_from_buffer(&buffer[cursor..])?;

        cursor += parsed_bytes;

        for (map_number, bitmap) in bitmaps.iter().enumerate() {
            for field_index in 1..=64 {
                let extra_add = (map_number * 64) as u8;
                if Self::is_field_in_bitmap(*bitmap, 64 - field_index) {
                    let key = (field_index + extra_add).to_string();
                    let (field_key, field) = BASE_ISO_FORMAT.get_key_value(key.as_str()).ok_or(
                        IsoMessageError::InvalidInput(format!(
                            "Unable to find field {key} in format"
                        )),
                    )?;

                    if field.is_bitmap() || message.contains_key(key.as_str()) {
                        continue;
                    }

                    let (field_value, bytes_parsed) =
                        field.get_value_from_buffer(&buffer[cursor..])?;

                    cursor += bytes_parsed;
                    message.insert(field_key, field_value);
                }
            }
        }

        Ok(message)
    }

    pub fn get_mti(&self) -> String {
        self.message.get("0").unwrap().to_owned()
    }

    pub fn get_type(&self) -> &IsoMessageType {
        ISO_MESSAGE_TYPE_MAP.get(self.get_mti().as_str()).unwrap()
    }

    pub fn get_required_field(&self, index: &str) -> Result<String, IsoMessageError> {
        Ok(self
            .message
            .get(index)
            .ok_or(IsoMessageError::InvalidInput(format!(
                "Unable to get field {index} from IsoMessage"
            )))?
            .clone())
    }

    pub fn get_optional_field(&self, index: &str) -> Option<String> {
        match self.message.get(index) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn set_field(&mut self, index: &'static str, value: &str) {
        self.message.insert(index, value.to_owned());
    }

    fn get_bit_maps(&self) -> Vec<u64> {
        let mut bitmaps = Vec::from([0_u64, 0_u64, 0_u64]);

        {
            let bitmap_ref = &mut bitmaps;

            self.message.iter().for_each(move |(key, _)| {
                if key == &"0" {
                    return;
                }
                let key = key.parse::<u8>().unwrap();

                let bit_map_index = if key % 64 == 0 { 64 } else { key % 64 };
                let bit_pos = if key % 64 == 0 {
                    (key / 64) as usize - 1
                } else {
                    (key / 64) as usize
                };

                if let Some(map) = bitmap_ref.get_mut(bit_pos) {
                    *map = *map | (1 << 64 - bit_map_index);
                }
            });
        }

        {
            let mut bitmap_iterator = bitmaps.iter_mut().peekable();

            while let Some(bitmap) = bitmap_iterator.next() {
                if let Some(next_bitmap) = bitmap_iterator.peek() {
                    if next_bitmap != &&0 {
                        *bitmap = *bitmap | 1 << 63;
                    }
                }
            }
        }

        bitmaps.into_iter().filter(|bitmap| bitmap != &0).collect()
    }

    pub fn get_message_buffer(&self) -> Result<Vec<u8>, IsoMessageError> {
        let bitmaps = self.get_bit_maps();

        let mut buffer: Vec<u8> = Vec::new();

        let mti = self.get_mti();

        buffer.extend_from_slice(mti.as_bytes());

        for map in bitmaps.iter() {
            buffer.extend_from_slice(&map.to_be_bytes());
        }

        for (map_index, map) in bitmaps.into_iter().enumerate() {
            for bit_index in 2..=64usize {
                if map & (1 << 64 - bit_index) == 0 {
                    continue;
                }
                let key = (bit_index + 64usize * map_index).to_string();
                let value = self.message.get(key.as_str()).ok_or_else(|| {
                    return IsoMessageError::InvalidInput(format!(
                        "Unable to get value from message for key: {key}. When converting to response message"
                    ));
                })?;
                let field = BASE_ISO_FORMAT.get(key.as_str()).ok_or_else(|| {
                    return IsoMessageError::InvalidInput(format!(
                        "Unable to get format of key: {}. When converting to response message",
                        key
                    ));
                })?;

                buffer.append(field.value_to_buffer(value).as_mut());
            }
        }

        let buffer_length = buffer.len().to_be_bytes();
        let mut buffer_length = buffer_length[(buffer_length.len() - 2)..].to_vec();
        buffer_length.append(&mut buffer);

        Ok(buffer_length)
    }
}

#[cfg(test)]
mod tests {

    mod get_bit_maps {
        use crate::{IsoJsonMessage, IsoMessage};

        #[test]
        fn should_get_1_bit_map() {
            let message = IsoJsonMessage::from([
                ("1", "".to_owned()),
                ("64", "".to_owned()),
                ("65", "".to_owned()),
                ("128", "".to_owned()),
                ("129", "".to_owned()),
                ("192", "".to_owned()),
            ]);

            let iso_message = IsoMessage::from_json_message(message);

            let bitmaps = iso_message.get_bit_maps();

            assert_eq!(
                bitmaps,
                [
                    9223372036854775809,
                    9223372036854775809,
                    9223372036854775809
                ]
            )
        }

        #[test]
        fn should_assemble_correct_bit_map() {
            let message = IsoJsonMessage::from([
                ("0", "".to_owned()),
                ("2", "".to_owned()),
                ("3", "".to_owned()),
                ("4", "".to_owned()),
                ("5", "".to_owned()),
                ("64", "".to_owned()),
            ]);

            let iso_message = IsoMessage::from_json_message(message);

            let bitmaps = iso_message.get_bit_maps();

            assert_eq!(bitmaps, [1])
        }
    }
}
