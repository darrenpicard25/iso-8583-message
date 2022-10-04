use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::field::Field;

const BASE_ISO_FORMAT_JSON: &'static str = include_str!("../formats/base-format.json");

lazy_static! {
    #[derive(Debug)]
    pub static ref BASE_ISO_FORMAT: HashMap<u8, Field> =
        serde_json::from_str(&BASE_ISO_FORMAT_JSON).unwrap();
}
