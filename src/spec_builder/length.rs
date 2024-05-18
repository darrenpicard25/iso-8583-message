use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "length_type", content = "length")]
#[serde(rename_all = "lowercase")]
pub enum LengthBuilder {
    #[serde(rename(deserialize = "fixed"))]
    Fixed(u16),
    #[serde(rename(deserialize = "lvar"))]
    LVar(u16),
    #[serde(rename(deserialize = "llvar"))]
    LLVar(u16),
    #[serde(rename(deserialize = "lllvar"))]
    LLLVar(u16),
}
