use std::collections::BTreeMap;

use super::data_element::DataElement;

#[derive(Debug)]
pub struct IsoSpecMap<'a>(BTreeMap<&'a str, DataElement<'a>>);

impl<'a> IsoSpecMap<'a> {
    pub fn new(map: BTreeMap<&'a str, DataElement<'a>>) -> Self {
        Self(map)
    }
}
