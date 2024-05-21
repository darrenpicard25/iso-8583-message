use std::{collections::BTreeMap, io::BufRead};

use crate::{iso_message::bitmap::Bitmap, IsoSpec};

mod bitmap;

pub struct IsoMessage<'a> {
    spec: &'a IsoSpec<'a>,
    data: BTreeMap<String, String>,
}

impl<'a> IsoMessage<'a> {
    pub fn new(spec: &'a IsoSpec, data: BTreeMap<String, String>) -> Self {
        Self { spec, data }
    }

    pub fn get_field(&self, key: &'static str) -> Option<&String> {
        let data_element_spec = self.spec.map.get_field_spec(key);

        let Some(_) = data_element_spec else {
            eprintln!("Attempting to get data element {key} that does not exist in spec");
            return None;
        };

        self.data.get(key)
    }

    pub fn set_field(&mut self, key: &'static str, value: String) -> Result<(), ()> {
        let data_element_spec = self.spec.map.get_field_spec(key);

        let Some(data_element_spec) = data_element_spec else {
            eprintln!("Attempting to set data element {key} that does not exist in spec");
            return Err(());
        };

        if !data_element_spec.is_valid(&value) {
            eprintln!("Data element {key} cannot be set for value passed in");
        }

        self.data.insert(key.to_string(), value);

        Ok(())
    }
}

impl<'a> TryFrom<&[u8]> for IsoMessage<'a> {
    type Error = std::io::Error;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        let mti = value.get(0..4);
        let bitmaps = Bitmap::try_from(&value[4..])?;

        todo!()
    }
}
