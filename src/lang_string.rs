// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LangString {
    pub language: String,
    pub text: String,
}

impl LangString {
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}

impl FromStr for LangString {
    type Err = ();

    /// The specification mentions an RDF/Turtle syntax that looks like this: "Hello"@en
    ///
    /// This implementation of from_str takes a &str in this syntax and returns the LangString or Err.
    ///
    /// # Example
    ///
    /// ```
    /// use std::str::FromStr;
    /// use basyx_rs::LangString;
    ///
    /// let expected = LangString::new("EN".to_string(), "Current temperature".to_string());
    /// let actual = LangString::from_str("\"Current temperature\"@EN").ok().unwrap();
    ///
    /// assert_eq!(actual, expected);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lang_string: Vec<&str> = s.split('@').collect();

        if lang_string.len() != 2 {
            Err(())
        } else {
            let mut txt = String::from(lang_string[0]);

            // remove first character (")
            if !txt.is_empty() {
                txt.remove(0);
            }

            // remove last character (")
            if !txt.is_empty() {
                txt.pop();
            }

            Ok(LangString {
                language: lang_string[1].to_string(),
                text: txt,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turtle_syntax() {
        // arrange
        let expected = LangString::new("EN".to_string(), "Current temperature".to_string());

        // act
        let actual = LangString::from_str("\"Current temperature\"@EN")
            .ok()
            .unwrap();

        // assert
        assert_eq!(actual, expected);
    }
}
