use serde::Deserialize;

#[derive(Deserialize)]
pub enum RedactionLevel {
    Full,
    None,
    Last4,
}
