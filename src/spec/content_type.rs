#[derive(Debug)]
pub enum ContentType {
    Alpha,
    Numeric,
    Special,
    Binary,
    AlphaNumeric,
    NumericSpecial,
    AlphaNumericSpecial,
}

impl ContentType {
    pub fn is_valid(&self, _value: &str) -> bool {
        true
    }
}
