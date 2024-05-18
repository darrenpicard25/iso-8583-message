use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ContentTypeBuilder {
    #[serde(rename(deserialize = "a"))]
    Alpha,
    #[serde(rename(deserialize = "n"))]
    Numeric,
    #[serde(rename(deserialize = "s"))]
    Special,
    #[serde(rename(deserialize = "b"))]
    Binary,
    #[serde(rename(deserialize = "an"))]
    AlphaNumeric,
    #[serde(rename(deserialize = "ns"))]
    NumericSpecial,
    #[serde(rename(deserialize = "ans"))]
    AlphaNumericSpecial,
}
