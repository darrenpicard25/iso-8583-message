use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ContentType {
    #[serde(rename = "b")]
    Bytes,
    #[serde(rename = "n")]
    Number,
    #[serde(rename = "p")]
    Padding,
    #[serde(rename = "a")]
    Alpha,
    #[serde(rename = "an")]
    AlphaNumeric,
    #[serde(rename = "anp")]
    AlphaNumericPadding,
    #[serde(rename = "s")]
    Special,
    #[serde(rename = "ans")]
    AlphaNumericSpecial,
    #[serde(rename = "z")]
    Track2,
    #[serde(rename = "x+n")]
    TransactionNumeric,
    #[serde(rename = "as")]
    AlphaSpecial,
    #[serde(rename = "ns")]
    NumericSpecial,
}
