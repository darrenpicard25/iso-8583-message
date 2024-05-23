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

impl<'a> DataElement<'a> {
    pub fn is_valid(&self, value: &str) -> bool {
        self.is_enabled && self.length.is_valid(value) && self.content_type.is_valid(value)
    }

    pub fn extract_from_buffer(&self, buffer: &[u8]) -> Result<(String, usize), std::io::Error> {
        todo!()
    }
}
