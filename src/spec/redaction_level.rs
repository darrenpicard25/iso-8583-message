#[derive(Debug)]
pub enum RedactionLevel {
    Full,
    None,
    Last4,
}

impl RedactionLevel {
    pub fn redact(&self, value: &str) -> String {
        match self {
            RedactionLevel::Full => value.chars().map(|_| '0').collect(),
            RedactionLevel::None => value.to_string(),
            RedactionLevel::Last4 => value
                .chars()
                .enumerate()
                .map(
                    |(index, char)| {
                        if index + 4 >= value.len() {
                            char
                        } else {
                            '0'
                        }
                    },
                )
                .collect(),
        }
    }
}

#[cfg(test)]
mod unit_test {
    mod redact {
        use crate::spec::redaction_level::RedactionLevel;

        #[test]
        fn redaction_level_of_full_should_fully_redact_value_when_called() {
            assert_eq!(
                RedactionLevel::Full.redact("String to redact"),
                "0000000000000000"
            )
        }

        #[test]
        fn redaction_level_of_none_should_not_redact_value_when_called() {
            assert_eq!(
                RedactionLevel::None.redact("String to redact"),
                "String to redact"
            )
        }

        #[test]
        fn redaction_level_of_last4_should_not_redact_last4_characters_of_value_when_called() {
            assert_eq!(
                RedactionLevel::Last4.redact("String to redact"),
                "000000000000dact"
            )
        }

        #[test]
        fn redaction_level_of_last4_should_not_redact_anything_if_value_is_less_then_4_characters()
        {
            assert_eq!(RedactionLevel::Last4.redact("car"), "car")
        }
    }
}
