use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq)]
// min: 6, max: 20 word: [0-9A-Za-z_]
pub struct UserName(String);

impl UserName {
    pub fn name(&self) -> &String {
        &self.0
    }
}

// [[:word:]] = [0-9A-Za-z_]
static WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[[:word:]]*$").expect("error parsing regex"));

impl TryFrom<String> for UserName {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, self::Error> {
        let v = value.graphemes(true).count();
        if !(6..=20).contains(&v) {
            return Err(anyhow!("UserId length must be between 6 to 20."));
        };
        if !WORD.is_match(&value) {
            return Err(anyhow!("UserId must be [0-9A-Za-z]"));
        };
        Ok(UserName(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_test() {
        // Correct
        {
            let n = "012345".to_string();
            let result = UserName::try_from(n.clone());
            assert!(result.is_ok());
            let user_id = result.unwrap();
            assert_eq!(user_id, UserName(n.clone()));
        }
        // Correct
        {
            let n = "01234567890123456789".to_string();
            let result = UserName::try_from(n.clone());
            assert!(result.is_ok());
            let user_id = result.unwrap();
            assert_eq!(user_id, UserName(n.clone()));
        }
        // Incorrect because value is less than 5 characters.
        {
            let user_id = UserName::try_from("01234".to_string());
            assert!(user_id.is_err());
        }
        // Incorrect because value is more than 21 characters.
        {
            let user_id = UserName::try_from("012345678901234567890".to_string());
            assert!(user_id.is_err());
        }
        // Incorrect because value has the characters out WORD.
        {
            let user_id = UserName::try_from("adf130_-*".to_string());
            assert!(user_id.is_err());
        }
    }
}
