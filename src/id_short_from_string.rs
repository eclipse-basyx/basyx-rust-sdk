// SPDX-FileCopyrightText: 2023 Jan Hecht
//
// SPDX-License-Identifier: MIT

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum IdShortErrorReason{
    TooShort,
    TooLong,
    NoMatch(String),
}

#[derive(Debug, PartialEq)]
pub struct IdShortError {
    reason: IdShortErrorReason
}

impl IdShortError {
    pub fn new(reason : IdShortErrorReason) -> IdShortError {
        IdShortError{reason: reason}
    }
}

/// id_short from &str
///
/// This function checks length and regex bounds, returning a Result.
pub fn id_short_from_str(hopefully_id_short : &str) -> Result<String, IdShortError>{
    id_short_from_string(hopefully_id_short.to_string())
}

/// id_short from String
///
/// This function checks length and regex bounds, returning a Result.
pub fn id_short_from_string(hopefully_id_short : String) -> Result<String, IdShortError>{
    if hopefully_id_short.len() < 1 {
        return Err(IdShortError::new(IdShortErrorReason::TooShort));
    }
    else if hopefully_id_short.len() > 128 {
        return Err(IdShortError::new(IdShortErrorReason::TooLong));
    }
    else{
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
        if re.is_match(hopefully_id_short.as_str()) {
            return Ok(hopefully_id_short);
        }
        else{
            return Err(IdShortError::new(IdShortErrorReason::NoMatch(hopefully_id_short)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_string_then_test_fails() {
        let result = id_short_from_str("");
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::TooShort)));
    }

    #[test]
    fn given_string_too_long_then_test_fails() {
        let result = id_short_from_str(
            "0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            012345678"
        ); // 12 * 10 + 9 = 129 (>128)
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::TooLong)));
    }

    #[test]
    fn given_length_max_then_test_passes() {
        // 12 * 10 + 8 = 128
        let input = "a123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            0123456789\
            01234567".to_string();

        let result = id_short_from_string(input.clone());
        assert_eq!(result, Ok(input));
    }

    #[test]
    fn given_first_character_when_question_mark_then_test_fails() {
        let input = "?Hello";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_first_character_when_underscore_then_test_fails() {
        let input = "?_Hello";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_foo_then_bla() {
        let result = id_short_from_str("my_new_id_short");
        assert_eq!(result, Ok(String::from("my_new_id_short")));
    }

    #[test]
    fn given_single_digit_then_test_fails() {
        let input = "1";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_string_len_1_lowercase_test_passes() {
        let result = id_short_from_str("a");
        assert_eq!(result, Ok("a".to_string()));
    }

    #[test]
    fn given_string_len_1_uppercase_test_passes() {
        let result = id_short_from_str("A");
        assert_eq!(result, Ok("A".to_string()));
    }

    // spaces
    #[test]
    fn given_space_then_test_fails() {
        let input = " ";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_leading_space_then_test_fails() {
        let input = " hello";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_trailing_space_then_test_fails() {
        let input = "hello ";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_space_in_middle_then_test_fails() {
        let input = "hello world";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }

    #[test]
    fn given_invalid_character_then_test_fails() {
        let input = "hello?world";
        let result = id_short_from_str(input);
        assert_eq!(result, Err(IdShortError::new(IdShortErrorReason::NoMatch(input.to_string()))));
    }
}