use anyhow::{anyhow, Error, Result};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Content(String);

impl Content {
    pub fn content(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for Content {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, self::Error> {
        if value.graphemes(true).count() > 200 {
            return Err(anyhow!("Content length must be 200 characters or less."));
        };
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_test() {
        // Correct
        {
            let s =
                "The Adventures of Tom Sawyer is set in the 1840's in the fictitious town of St.Petersburg, Missouri, where Tom lives with his deceased mother's sister,Aunt Polly, and his half-brother, Sid. After To".to_string();
            let result = Content::try_from(s);
            assert!(result.is_ok());
        }
        // Correct
        {
            let s = "あいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてと".to_string();
            let result = Content::try_from(s);
            assert!(result.is_ok());
        }
        // Incorrect because value is longer than 201 characters.
        {
            let s =
                "The Adventures of Tom Sawyer is set in the 1840's in the fictitious town of St.Petersburg, Missouri, where Tom lives with his deceased mother's sister,Aunt Polly, and his half-brother, Sid. After Too s".to_string();
            let result = Content::try_from(s);
            assert!(result.is_err())
        }
        // Incorrect because value is longer than 201 characters.
        {
            let s = "あいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあいうえおかきくけこさしすせそたちつてとあ".to_string();
            let result = Content::try_from(s);
            assert!(result.is_err())
        }
    }
}
