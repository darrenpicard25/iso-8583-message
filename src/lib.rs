use std::collections::HashMap;

use bitmaps::Bitmaps;
use error::IsoMessageError;
use format::BASE_ISO_FORMAT;
use response::RESPONSE_FIELDS;

mod bitmaps;
mod error;
mod field;
mod format;
mod response;

type IsoMessageMap = HashMap<u8, String>;

const REQUEST_MESSAGE_TYPES: [&str; 9] = [
    "0100", "0120", "0200", "0220", "0302", "0400", "0420", "0620", "0800",
];
const RESPONSE_MESSAGE_TYPES: [&str; 9] = [
    "0110", "0130", "0210", "0230", "0312", "0410", "0430", "0630", "0810",
];

pub struct IsoMessage {
    original_buffer: Option<Vec<u8>>,
    map: IsoMessageMap,
}

impl IsoMessage {
    pub fn from_buffer(buffer: Vec<u8>) -> Result<Self, IsoMessageError> {
        let mut cursor: usize = 0;
        let mut map = IsoMessageMap::new();
        let (mti, bytes_parsed) = BASE_ISO_FORMAT
            .get(&0)
            .ok_or(IsoMessageError::InvalidInput(0))?
            .get_value_from_buffer(&buffer[cursor..])?;

        cursor += bytes_parsed;
        map.insert(0, mti);

        let bitmaps = Bitmaps::from_buffer(&buffer[cursor..]);
        cursor += bitmaps.byte_length();

        for field_key in bitmaps.items() {
            if field_key == 1 || field_key == 65 {
                continue;
            }

            let field = BASE_ISO_FORMAT
                .get(&field_key)
                .ok_or(IsoMessageError::InvalidInput(field_key))?;

            let (value, bytes_parsed) = field.get_value_from_buffer(&buffer[cursor..])?;

            cursor += bytes_parsed;
            map.insert(field_key, value);
        }

        Ok(Self {
            map,
            original_buffer: Some(buffer),
        })
    }

    pub fn from_iso_json_message(map: IsoMessageMap) -> Self {
        Self {
            map,
            original_buffer: None,
        }
    }

    pub fn to_response(self, response_code: &'static str) -> Result<Self, IsoMessageError> {
        if self.is_response() {
            return Ok(self);
        }

        let mti = self.get_mti().ok_or(IsoMessageError::InvalidInput(0))?;
        let mti = mti
            .to_owned()
            .parse::<u32>()
            .map_err(|_| IsoMessageError::InvalidInput(0))?;
        let mti = format!("{:0<4}", (mti + 10));

        let fields_to_keep_list = RESPONSE_FIELDS.get(mti.as_str());
        let iso_message = self.set_field(0, mti).set_field(39, response_code.into());

        if let Some(fields_to_keep) = fields_to_keep_list {
            let fields_to_remove: Vec<u8> = iso_message
                .get_message_map()
                .keys()
                .filter(|key| !fields_to_keep.contains(*key))
                .map(|key| *key)
                .collect();

            let iso_message = fields_to_remove
                .iter()
                .fold(iso_message, |acc, field| acc.delete_field(*field));

            return Ok(iso_message);
        } else {
            return Ok(iso_message);
        }
    }

    pub fn get_mti(&self) -> Option<&str> {
        self.get_field(0)
    }

    pub fn get_pan(&self) -> Option<&str> {
        self.get_field(2)
    }

    pub fn get_original_buffer(&self) -> &Option<Vec<u8>> {
        &self.original_buffer
    }

    pub fn get_field(&self, index: u8) -> Option<&str> {
        self.map.get(&index).map(|value| value.as_str())
    }

    pub fn set_field(mut self, index: u8, value: String) -> Self {
        self.map.insert(index, value);

        self
    }

    pub fn delete_field(mut self, index: u8) -> Self {
        self.map.remove(&index);

        self
    }

    pub fn is_request(&self) -> bool {
        if let Some(mti) = self.get_mti() {
            REQUEST_MESSAGE_TYPES.contains(&mti)
        } else {
            false
        }
    }

    pub fn is_response(&self) -> bool {
        if let Some(mti) = self.get_mti() {
            RESPONSE_MESSAGE_TYPES.contains(&mti)
        } else {
            false
        }
    }

    pub fn get_message_buffer(&self) -> Result<Vec<u8>, IsoMessageError> {
        let mut buffer: Vec<u8> = Vec::new();

        // MTI
        let mti = self.get_mti().ok_or(IsoMessageError::InvalidInput(0))?;
        buffer.extend_from_slice(mti.as_bytes());

        // Bitmaps
        let bitmaps = Bitmaps::from_iso_message_map(&self.map);
        buffer.extend(bitmaps.to_buffer());

        // Fields
        for key in bitmaps.items() {
            let field = BASE_ISO_FORMAT
                .get(&key)
                .ok_or(IsoMessageError::InvalidInput(key))?;

            let value = self
                .map
                .get(&key)
                .expect(&format!("Unable to get DE:{key}"));

            buffer.append(field.value_to_buffer(value)?.as_mut());
        }

        Ok(buffer)
    }

    pub fn get_message_map(&self) -> &IsoMessageMap {
        &self.map
    }
}
