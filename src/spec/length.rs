use serde::Deserialize;

#[derive(Deserialize)]
pub enum Length {
    Fixed(u16),
    LVar(u16),
    LLVar(u16),
    LLLVar(u16),
}