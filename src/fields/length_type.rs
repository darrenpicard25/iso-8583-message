use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum LengthType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "lvar")]
    Var1Leading,
    #[serde(rename = "llvar")]
    Var2Leading,
    #[serde(rename = "lllvar")]
    Var3Leading,
    #[serde(rename = "llllvar")]
    Var4Leading,
    #[serde(rename = "lllllvar")]
    Var5Leading,
    #[serde(rename = "llllllvar")]
    Var6Leading,
}

impl LengthType {
    pub fn get_leading_digits(&self) -> Option<usize> {
        match self {
            LengthType::Fixed => None,
            LengthType::Var1Leading => Some(1),
            LengthType::Var2Leading => Some(2),
            LengthType::Var3Leading => Some(3),
            LengthType::Var4Leading => Some(4),
            LengthType::Var5Leading => Some(5),
            LengthType::Var6Leading => Some(6),
        }
    }
}
