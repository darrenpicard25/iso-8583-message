use serde::Deserialize;

#[derive(Deserialize)]
pub enum ContentType {
    Alpha,
    Numeric,
    Special,
    Binary,
    AlphaNumeric,
    NumericSpecial,
    AlphaNumericSpecial,
}
