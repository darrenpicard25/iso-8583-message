use std::collections::BTreeMap;

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

    pub fn from_buffer(buffer: &[u8], spec: &'a IsoSpec<'a>) -> Result<Self, std::io::Error> {
        let mut data: BTreeMap<String, String> = BTreeMap::new();
        let mut cursor = 0;
        let mti = buffer
            .get(cursor..cursor + 4)
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unable to extract mti from buffer",
            ))?;
        cursor += 4;
        data.insert("0".to_string(), mti);

        let bitmaps = Bitmap::try_from(&buffer[cursor..])?;
        cursor += bitmaps.bytes_consumed();

        for key in bitmaps.into_iter() {
            let key = key.to_string();
            let field_spec = spec
                .map
                .get_field_spec(key.as_str())
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Unable to find spec for data element: {key}"),
                ))?;

            let (value, bytes_consumed) = field_spec.extract_from_buffer(&buffer[cursor..])?;
            cursor += bytes_consumed;

            data.insert(key.to_string(), value);
        }

        Ok(Self { spec, data })
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
