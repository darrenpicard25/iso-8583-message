use std::collections::HashMap;

mod error;
mod field;
mod format;

pub type IsoMessageMap = HashMap<&'static str, String>;

pub struct IsoMessage {
    original_buffer: Option<Vec<u8>>,
    map: IsoMessageMap,
}

impl IsoMessage {
    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        todo!()
    }

    pub fn from_iso_json_message(message: IsoMessageMap) -> Self {
        todo!()
    }

    pub fn get_mti(&self) -> &str {
        todo!()
    }

    pub fn get_pan(&self) -> &str {
        todo!()
    }

    pub fn get_field(&self, index: &str) -> Option<&str> {
        todo!();
    }

    pub fn set_field(self, index: &str, value: String) -> Self {
        todo!()
    }

    pub fn delete_field(self, index: &str) -> Self {
        todo!();
    }

    pub fn get_message_buffer(&self) -> Vec<u8> {
        todo!()
    }

    pub fn get_message_map(&self) -> IsoMessageMap {
        todo!()
    }
}
