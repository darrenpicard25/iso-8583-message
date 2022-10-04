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

#[cfg(test)]
mod tests {
    use super::LengthType;

    mod get_leading_digits {
        use super::LengthType;

        #[test]
        fn should_return_none_if_length_type_is_fixed() {
            assert_eq!(LengthType::Fixed.get_leading_digits(), None);
        }

        #[test]
        fn should_return_1_if_length_type_is_lvar() {
            assert_eq!(LengthType::Var1Leading.get_leading_digits(), Some(1));
        }

        #[test]
        fn should_return_1_if_length_type_is_llvar() {
            assert_eq!(LengthType::Var2Leading.get_leading_digits(), Some(2));
        }

        #[test]
        fn should_return_1_if_length_type_is_lllvar() {
            assert_eq!(LengthType::Var3Leading.get_leading_digits(), Some(3));
        }

        #[test]
        fn should_return_1_if_length_type_is_llllvar() {
            assert_eq!(LengthType::Var4Leading.get_leading_digits(), Some(4));
        }
    }
}
