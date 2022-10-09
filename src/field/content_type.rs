use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
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

const ALPHA: &str = "A-Za-z";
const PADDING: &str = "*#\x20 ";
const NUMERIC: &str = "0-9";
const SPECIAL: &str = r#"-!$%^&*()_+|~=`{}\[\]:";'<>?,\./ \\"#;

lazy_static! {
    static ref BYTES_REGEX: Regex = Regex::new("^[A-Fa-f0-9]+$").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(format!("^[{NUMERIC}]+$").as_str()).unwrap();
    static ref PADDING_REGEX: Regex = Regex::new(format!("^[{PADDING}]+$").as_str()).unwrap();
    static ref ALPHA_REGEX: Regex = Regex::new(format!("^[{ALPHA}]+$").as_str()).unwrap();
    static ref ALPHA_NUMERIC_REGEX: Regex =
        Regex::new(format!("^[{ALPHA}{NUMERIC}]+$").as_str()).unwrap();
    static ref ALPHA_NUMERIC_PADDING_REGEX: Regex =
        Regex::new(format!("^[{ALPHA}{NUMERIC}{PADDING}]+$").as_str()).unwrap();
    static ref SPECIAL_REGEX: Regex = Regex::new(format!("^[{SPECIAL}]+$").as_str()).unwrap();
    static ref ALPHA_NUMERIC_SPECIAL_REGEX: Regex =
        Regex::new(format!("^[{ALPHA}{NUMERIC}{SPECIAL}]+$").as_str()).unwrap();
    static ref TRANSACTION_NUMERIC_REGEX: Regex =
        Regex::new(format!("^[C|D][{NUMERIC}]+$").as_str()).unwrap();
    static ref ALPHA_SPECIAL_REGEX: Regex =
        Regex::new(format!("^[{ALPHA}{SPECIAL}]+$").as_str()).unwrap();
    static ref NUMERIC_SPECIAL_REGEX: Regex =
        Regex::new(format!("^[{NUMERIC}{SPECIAL}]+$").as_str()).unwrap();
}

impl ContentType {
    pub fn is_valid(&self, value: &str) -> bool {
        match self {
            ContentType::Bytes => BYTES_REGEX.is_match(value),
            ContentType::Number => NUMBER_REGEX.is_match(value),
            ContentType::Padding => PADDING_REGEX.is_match(value),
            ContentType::Alpha => ALPHA_REGEX.is_match(value),
            ContentType::AlphaNumeric => ALPHA_NUMERIC_REGEX.is_match(value),
            ContentType::AlphaNumericPadding => ALPHA_NUMERIC_PADDING_REGEX.is_match(value),
            ContentType::Special => SPECIAL_REGEX.is_match(value),
            ContentType::AlphaNumericSpecial => ALPHA_NUMERIC_SPECIAL_REGEX.is_match(value),
            ContentType::Track2 => true,
            ContentType::TransactionNumeric => TRANSACTION_NUMERIC_REGEX.is_match(value),
            ContentType::AlphaSpecial => ALPHA_SPECIAL_REGEX.is_match(value),
            ContentType::NumericSpecial => NUMERIC_SPECIAL_REGEX.is_match(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContentType;
    mod is_valid {
        use super::ContentType;
        mod content_type_bytes {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_hex() {
                assert!(ContentType::Bytes.is_valid("123456789ABCDEF"));
            }

            #[test]
            fn should_return_false_if_value_not_hex() {
                assert!(!ContentType::Bytes.is_valid("12345678-9ABCDEF"));
            }
        }

        mod content_type_number {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_number() {
                assert!(ContentType::Number.is_valid("123456789"));
            }

            #[test]
            fn should_return_false_if_value_not_number() {
                assert!(!ContentType::Number.is_valid("9ABCDEF"));
            }
        }

        mod content_type_padding {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_padding() {
                assert!(ContentType::Padding.is_valid("  **##"));
            }

            #[test]
            fn should_return_false_if_value_not_number() {
                assert!(!ContentType::Padding.is_valid("9ABCDEF"));
            }
        }

        mod content_type_alpha {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_alpha() {
                assert!(ContentType::Alpha.is_valid("helloWorld"));
            }

            #[test]
            fn should_return_false_if_value_not_alpha() {
                assert!(!ContentType::Alpha.is_valid("9ABCDEF"));
            }
        }

        mod content_type_alpha_numeric {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_alpha_numeric() {
                assert!(ContentType::AlphaNumeric.is_valid("helloWorld123456"));
            }

            #[test]
            fn should_return_false_if_value_not_alpha_numeric() {
                assert!(!ContentType::Alpha.is_valid("9ABCDEF "));
            }
        }

        mod content_type_alpha_numeric_padding {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_alpha_numeric_padding() {
                assert!(ContentType::AlphaNumericPadding.is_valid("helloWorld123456   *#"));
            }

            #[test]
            fn should_return_false_if_value_not_alpha_numeric_padding() {
                assert!(!ContentType::Alpha.is_valid("9ABCDEF -- "));
            }
        }

        mod content_type_special {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_special() {
                assert!(ContentType::Special.is_valid("--__"));
            }

            #[test]
            fn should_return_false_if_value_not_special() {
                assert!(!ContentType::Special.is_valid("9ABCDEF -- "));
            }
        }

        mod content_type_alpha_numeric_special {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_alpha_numeric_special() {
                assert!(ContentType::AlphaNumericSpecial.is_valid("abcdhe38586_-92hsn"));
            }

            #[test]
            fn should_return_false_if_value_not_special() {
                assert!(!ContentType::AlphaNumericSpecial.is_valid("9ABCDEF --# "));
            }
        }

        mod content_type_track_2 {
            use super::ContentType;

            #[test]
            fn should_always_return_true() {
                assert!(ContentType::Track2.is_valid("abcdhe38586_-92hsn"));
            }
        }

        mod content_type_transaction_numeric {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_transaction_numeric() {
                assert!(ContentType::TransactionNumeric.is_valid("3499"));
            }

            #[test]
            fn should_return_false_if_value_not_special() {
                assert!(!ContentType::TransactionNumeric.is_valid("3499A"));
            }
        }

        mod content_type_alpha_special {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_alpha_special() {
                assert!(ContentType::AlphaSpecial.is_valid("adb-akdnA"));
            }

            #[test]
            fn should_return_false_if_value_not_special() {
                assert!(!ContentType::AlphaSpecial.is_valid("adb-asDF2"));
            }
        }

        mod content_type_numeric_special {
            use super::ContentType;

            #[test]
            fn should_return_true_if_value_is_numeric_special() {
                assert!(ContentType::NumericSpecial.is_valid("123.44"));
            }

            #[test]
            fn should_return_false_if_value_not_numeric_special() {
                assert!(!ContentType::NumericSpecial.is_valid("123.44A"));
            }
        }
    }
}
