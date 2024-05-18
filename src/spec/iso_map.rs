use std::collections::BTreeMap;

use super::data_element::DataElement;

pub struct IsoSpecMap<'a>(BTreeMap<&'a str, DataElement<'a>>);
