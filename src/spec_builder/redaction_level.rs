use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RedactionLevelBuilder {
    Full,
    None,
    Last4,
}

impl Default for RedactionLevelBuilder {
    fn default() -> Self {
        Self::Full
    }
}
