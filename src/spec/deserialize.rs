use std::collections::BTreeMap;

use serde::{
    de::{self, Visitor},
    Deserialize,
};

use crate::spec::{data_element::DataElement, length::Length};

use super::{
    content_type::ContentType, iso_map::IsoSpecMap, redaction_level::RedactionLevel, IsoSpec,
};

impl<'de> Deserialize<'de> for RedactionLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RedactionLevelVisitor;

        impl<'de> Visitor<'de> for RedactionLevelVisitor {
            type Value = RedactionLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RedactionLevel")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "last4" => Ok(RedactionLevel::Last4),
                    "none" => Ok(RedactionLevel::None),
                    "full" => Ok(RedactionLevel::Full),
                    unexpected => Err(de::Error::unknown_variant(
                        unexpected,
                        &["last4", "none", "full"],
                    )),
                }
            }
        }

        deserializer.deserialize_str(RedactionLevelVisitor)
    }
}

impl<'de> Deserialize<'de> for IsoSpecMap<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IsoSpecMapVisitor;

        impl<'de> Visitor<'de> for IsoSpecMapVisitor {
            type Value = IsoSpecMap<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a very special map")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut map = BTreeMap::new();

                while let Some((key, value)) = access.next_entry::<&str, DataElement<'de>>()? {
                    map.insert(key, value);
                }

                Ok(IsoSpecMap::new(map))
            }
        }

        deserializer.deserialize_map(IsoSpecMapVisitor)
    }
}

impl<'de> Deserialize<'de> for DataElement<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Label,
            Description,
            ContentType,
            RedactionLevel,
            Length,
            LengthType,
            Disabled,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(
                            "label, description, content_type, redaction_level, or length needed",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "label" => Ok(Field::Label),
                            "description" => Ok(Field::Description),
                            "content_type" => Ok(Field::ContentType),
                            "redaction_level" => Ok(Field::RedactionLevel),
                            "length" => Ok(Field::Length),
                            "length_type" => Ok(Field::LengthType),
                            "disabled" => Ok(Field::Disabled),
                            unexpected => Err(de::Error::unknown_field(unexpected, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DataElementVisitor;

        impl<'de> Visitor<'de> for DataElementVisitor {
            type Value = DataElement<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct DataElement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut description = None;
                let mut label = None;
                let mut length = None;
                let mut content_type = None;
                let mut length_type = None;
                let mut redaction_level = None;
                let mut disabled = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Label => {
                            label = Some(map.next_value()?);
                        }
                        Field::Description => {
                            description = Some(map.next_value()?);
                        }
                        Field::ContentType => {
                            content_type = Some(map.next_value()?);
                        }
                        Field::RedactionLevel => {
                            redaction_level = Some(map.next_value()?);
                        }
                        Field::Length => {
                            length = Some(map.next_value()?);
                        }
                        Field::LengthType => {
                            length_type = Some(map.next_value()?);
                        }
                        Field::Disabled => {
                            disabled = Some(map.next_value()?);
                        }
                    }
                }

                let label = label.ok_or(de::Error::missing_field("label"))?;
                let description = description.ok_or(de::Error::missing_field("description"))?;
                let redaction_level =
                    redaction_level.ok_or(de::Error::missing_field("redaction_level"))?;
                let content_type = content_type.ok_or(de::Error::missing_field("content_type"))?;
                let is_enabled = !disabled.unwrap_or(false);

                let length = length.ok_or(de::Error::missing_field("length"))?;
                let length = match length_type.ok_or(de::Error::missing_field("length_type"))? {
                    "fixed" => Ok(Length::Fixed(length)),
                    "lvar" => Ok(Length::LVar(length)),
                    "llvar" => Ok(Length::LLVar(length)),
                    "lllvar" => Ok(Length::LLLVar(length)),
                    unexpected => Err(de::Error::invalid_value(
                        de::Unexpected::Str(&unexpected),
                        &"fixed, lvar, llvar, lllvar",
                    )),
                }?;

                Ok(DataElement {
                    label,
                    description,
                    redaction_level,
                    content_type,
                    length,
                    is_enabled,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "label",
            "description",
            "content_type",
            "redaction_level",
            "length",
            "length_type",
            "disabled",
        ];

        deserializer.deserialize_struct("DataElement", FIELDS, DataElementVisitor)
    }
}

impl<'de> Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ContentTypeVisitor;

        impl<'de> Visitor<'de> for ContentTypeVisitor {
            type Value = ContentType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContentType")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "n" => Ok(ContentType::Numeric),
                    "a" => Ok(ContentType::Alpha),
                    "an" => Ok(ContentType::AlphaNumeric),
                    "ans" => Ok(ContentType::AlphaNumericSpecial),
                    "b" => Ok(ContentType::Binary),
                    "s" => Ok(ContentType::Special),
                    "ns" => Ok(ContentType::NumericSpecial),
                    unexpected => Err(de::Error::unknown_variant(
                        &unexpected,
                        &["n", "a", "an", "ans", "b", "s", "ns"],
                    )),
                }
            }
        }

        deserializer.deserialize_str(ContentTypeVisitor)
    }
}

impl<'de> Deserialize<'de> for IsoSpec<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Name,
            Version,
            Map,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("name, version, map expected")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        dbg!(value);
                        match value {
                            "name" => Ok(Field::Name),
                            "version" => Ok(Field::Version),
                            "map" => Ok(Field::Map),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct IsoSpecVisitor;

        impl<'de> Visitor<'de> for IsoSpecVisitor {
            type Value = IsoSpec<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct IsoSpec")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut name = None;
                let mut version = None;
                let mut map = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        Field::Name => name = Some(access.next_value()?),
                        Field::Version => version = Some(access.next_value()?),
                        Field::Map => map = Some(access.next_value()?),
                    }
                }

                Ok(IsoSpec {
                    name: name.ok_or(de::Error::missing_field("name"))?,
                    version: version.ok_or(de::Error::missing_field("version"))?,
                    map: map.ok_or(de::Error::missing_field("map"))?,
                })
            }
        }

        const FIELDS: &[&str] = &["name", "version", "map"];

        deserializer.deserialize_struct("IsoSpec", FIELDS, IsoSpecVisitor)
    }
}
