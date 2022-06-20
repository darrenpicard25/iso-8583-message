use base_format::BASE_ISO_FORMAT;
use message_types::{IsoMessageType, ISO_MESSAGE_TYPE_MAP};
use std::collections::HashMap;

mod base_format;
mod fields;
mod message_types;

pub type IsoJsonMessage = HashMap<String, String>;

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

        let get_bitmap = move |map_name: &str| -> Result<u64, IsoMessageError> {
            Ok(u64::from_be_bytes(
                buffer[total_bytes..total_bytes + 8]
                    .try_into()
                    .map_err(|_| {
                        IsoMessageError::InvalidInput(format!("Unable to get {map_name} bitmap"))
                    })?,
            ))
        };
        let primary_bitmap = get_bitmap("primary")?;
        bitmaps.push(primary_bitmap);
        total_bytes += 8;

        if Self::is_field_in_bitmap(primary_bitmap, 64 - 1) {
            let secondary_bitmap = get_bitmap("secondary")?;
            bitmaps.push(secondary_bitmap);
            total_bytes += 8;

            if Self::is_field_in_bitmap(secondary_bitmap, 64 - 1) {
                let tertiary_bitmap = get_bitmap("tertiary")?;
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

        let (mti, parsed_bytes) = BASE_ISO_FORMAT
            .get("0")
            .ok_or(IsoMessageError::InvalidInput(
                "Unable to find field 0 in format".to_owned(),
            ))?
            .get_value_from_buffer(&buffer)?;
        cursor += parsed_bytes;
        message.insert("0".to_owned(), mti);

        let (bitmaps, parsed_bytes) = IsoMessage::get_bitmaps_from_buffer(&buffer[cursor..])?;
        cursor += parsed_bytes;

        for (map_number, bitmap) in bitmaps.iter().enumerate() {
            for field_index in 1..=64 {
                let extra_add = (map_number * 64) as u8;
                if Self::is_field_in_bitmap(*bitmap, 64 - field_index) {
                    let key = (field_index + extra_add).to_string();
                    let field =
                        BASE_ISO_FORMAT
                            .get(key.as_str())
                            .ok_or(IsoMessageError::InvalidInput(format!(
                                "Unable to find field {key} in format"
                            )))?;

                    if field.is_bitmap() || message.contains_key(&key) {
                        continue;
                    }

                    let (field_value, bytes_parsed) =
                        field.get_value_from_buffer(&buffer[cursor..])?;
                    cursor += bytes_parsed;
                    message.insert(key.to_owned(), field_value);
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

    pub fn set_field(&mut self, index: &str, value: &str) {
        self.message.insert(index.to_owned(), value.to_owned());
    }

    fn get_bit_maps(&self) -> Vec<u64> {
        let mut bitmaps = Vec::from([0_u64, 0_u64, 0_u64]);

        {
            let bitmap_ref = &mut bitmaps;

            self.message.iter().for_each(move |(key, _)| {
                let key = key.parse::<u8>().unwrap();

                let bit_pos = (key / 64) as usize;
                let bit_map_index = key % 64;

                if let Some(map) = bitmap_ref.get_mut(bit_pos) {
                    *map = *map | (1 << 64 - bit_map_index)
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

        bitmaps
            .into_iter()
            .for_each(|map| buffer.extend_from_slice(&map.to_be_bytes()));

        for (key, value) in self.message.iter() {
            if key == "2" {
                continue;
            }

            let field = BASE_ISO_FORMAT.get(key.as_str()).ok_or_else(|| {
                return IsoMessageError::InvalidInput(format!(
                    "Unable to get format of key: {}. When converting to response message",
                    key
                ));
            })?;

            buffer.append(field.value_to_buffer(value).as_mut())
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {

    mod get_bit_maps {
        use crate::{IsoJsonMessage, IsoMessage};

        #[test]
        fn should_get_1_bit_map() {
            let message = IsoJsonMessage::from([
                ("2".to_owned(), "".to_owned()),
                ("10".to_owned(), "".to_owned()),
                ("15".to_owned(), "".to_owned()),
                ("65".to_owned(), "".to_owned()),
            ]);

            let iso_message = IsoMessage::from_json_message(message);

            let bitmaps = iso_message.get_bit_maps();

            bitmaps.iter().for_each(|map| {
                println!("{:064b}", map);
            });

            assert_eq!(bitmaps, [0_u64])
        }
    }
    #[test]
    fn bit_shifting() {
        let mut value: u64 = 1;

        value = value | (1 << 3);

        assert_eq!(value, 9);
    }
}
