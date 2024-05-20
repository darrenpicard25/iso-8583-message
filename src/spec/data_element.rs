use super::{content_type::ContentType, length::Length, redaction_level::RedactionLevel};

#[derive(Debug)]
pub struct DataElement<'a> {
    pub label: &'a str,
    pub description: &'a str,
    pub is_enabled: bool,
    pub content_type: ContentType,
    pub redaction_level: RedactionLevel,
    pub length: Length,
}
