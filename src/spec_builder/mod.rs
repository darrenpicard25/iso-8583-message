use std::collections::BTreeMap;

use serde::Deserialize;

use crate::spec::{iso_map::IsoSpecMap, IsoSpec};

use self::{
    content_type::ContentTypeBuilder, length::LengthBuilder, redaction_level::RedactionLevelBuilder,
};

mod content_type;
mod length;
mod redaction_level;

#[derive(Debug, Deserialize)]
pub struct IsoSpecBuilder<'a> {
    name: &'a str,
    version: u16,
    map: IsoSpecMapBuilder<'a>,
}

impl<'a> TryFrom<&'a str> for IsoSpecBuilder<'a> {
    type Error = serde_json::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl<'a> From<IsoSpecBuilder<'a>> for IsoSpec<'a> {
    fn from(value: IsoSpecBuilder<'a>) -> Self {
        IsoSpec {
            name: value.name,
            version: value.version,
            map: value.map.into(),
        }
    }
}

#[derive(Debug)]
pub struct IsoSpecMapBuilder<'a>(BTreeMap<&'a str, DataElementBuilder<'a>>);

impl<'a> From<IsoSpecMapBuilder<'a>> for IsoSpecMap<'a> {
    fn from(value: IsoSpecMapBuilder<'a>) -> Self {
        todo!()
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for IsoSpecMapBuilder<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(IsoSpecMapBuilder(BTreeMap::deserialize(deserializer)?))
    }
}

#[derive(Deserialize, Debug)]
pub struct DataElementBuilder<'a> {
    label: &'a str,
    description: &'a str,
    content_type: ContentTypeBuilder,
    redaction_level: RedactionLevelBuilder,
    #[serde(flatten)]
    length: LengthBuilder,
}
