use serde::Deserialize;

use super::{content_type::ContentType, length::Length, redaction_level::RedactionLevel};

pub struct DataElement<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub content_type: ContentType,
    pub redaction_level: RedactionLevel,
    pub length: Length,
}

impl<'a> Deserialize<'a> for DataElement<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let inner_struct = deserializer.deserialize_struct(
            "DataElement",
            &["label", "description", "content_type"],
            serde::de::Visitor::visit_borrowed_str,
        )?;
    }
}
