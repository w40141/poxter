use anyhow::{anyhow, Error, Result};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct ProfileName(String);

impl ProfileName {
    pub fn get(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for ProfileName {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, self::Error> {
        if value.graphemes(true).count() > 50 {
            return Err(anyhow!("Content length must be 50 characters or less."));
        };
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_name_test() {
        // Correct
        {
            let s = "The Adventures of Tom Sawyer is set in the 1840'sa".to_string();
            let result = ProfileName::try_from(s);
            assert!(result.is_ok());
        }
        // Correct
        {
            let s = "あいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこ".to_string();
            let result = ProfileName::try_from(s);
            assert!(result.is_ok());
        }
        // Incorrect because value is longer than 201 characters.
        {
            let s = "The Adventures of Tom Sawyer is set in the 1840's in ".to_string();
            let result = ProfileName::try_from(s);
            assert!(result.is_err())
        }
        // Incorrect because value is longer than 201 characters.
        {
            let s = "あいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさ".to_string();
            let result = ProfileName::try_from(s);
            assert!(result.is_err())
        }
    }
}
