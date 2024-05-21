#[derive(Debug)]
pub enum Length {
    Fixed(u16),
    LVar(u16),
    LLVar(u16),
    LLLVar(u16),
}

impl Length {
    pub fn is_valid(&self, value: &str) -> bool {
        match self {
            Length::Fixed(length) => *length as usize == value.len(),
            Length::LVar(max_len) => *max_len as usize >= value.len(),
            Length::LLVar(max_len) => *max_len as usize >= value.len(),
            Length::LLLVar(max_len) => *max_len as usize >= value.len(),
        }
    }
}
