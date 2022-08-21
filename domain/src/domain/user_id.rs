use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct UserId {
    // min: 6, max: 20 word: [0-9A-Za-z_]
    value: String,
}

impl UserId {
    pub fn value(&self) -> &String {
        &self.value
    }
}

// [[:word:]] = [0-9A-Za-z_]
static WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[[:word:]]*$").expect("error parsing regex"));

impl TryFrom<String> for UserId {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, self::Error> {
        if value.len() < 6 || value.len() > 20 {
            return Err(anyhow!("UserId length must be between 6 to 20."));
        };
        if !WORD.is_match(&value) {
            return Err(anyhow!("UserId must be [0-9A-Za-z]"));
        };
        Ok(UserId { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_test() {
        {
            // Correct
            let user_id = UserId::try_from("adf130_".to_string());
            assert!(user_id.is_ok());
        }
        {
            // Incorrect because value is less than 5 characters.
            let user_id = UserId::try_from("adf1".to_string());
            assert!(user_id.is_err());
        }
        {
            // Incorrect because value has the characters out WORD.
            let user_id = UserId::try_from("adf130_-*".to_string());
            assert!(user_id.is_err());
        }
    }
}