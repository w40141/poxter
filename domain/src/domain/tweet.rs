use anyhow::{anyhow, Error, Result};
use unicode_segmentation::UnicodeSegmentation;

use crate::domain::user_id::UserId;

#[derive(Debug, Clone)]
pub struct Tweet {
    user_id: UserId,
    content: Content,
}

impl Tweet {
    pub fn user_id(&self) -> &String {
        self.user_id.value()
    }

    pub fn content(&self) -> &String {
        self.content.value()
    }
}

#[derive(Debug, Clone)]
pub struct TweetBuilder {
    user_id: Option<UserId>,
    content: Option<Content>,
}

impl TweetBuilder {
    pub fn default() -> Self {
        Self {
            user_id: None,
            content: None,
        }
    }

    pub fn user_id(mut self, v: UserId) -> Self {
        self.user_id = Some(v);
        self
    }

    pub fn content(mut self, v: Content) -> Self {
        self.content = Some(v);
        self
    }

    pub fn build(&self) -> Result<Tweet> {
        let user_id = match &self.user_id {
            Some(v) => v.clone(),
            None => return Err(anyhow!("NotFound user_id.")),
        };

        let content = match &self.content {
            Some(v) => v.clone(),
            None => return Err(anyhow!("NotFound content.")),
        };

        Ok(Tweet { user_id, content })
    }
}

#[derive(Debug, Clone)]
pub struct Content {
    value: String,
}

impl Content {
    pub fn value(&self) -> &String {
        &self.value
    }
}

impl TryFrom<String> for Content {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, self::Error> {
        if value.graphemes(true).count() > 200 {
            return Err(anyhow!("Content length must be 200 characters or less."));
        };
        Ok(Self { value })
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

    #[test]
    fn tweet_test() {
        {
            let result = TweetBuilder::default()
                .user_id(UserId::try_from("taro1010".to_string()).unwrap())
                .content(Content::try_from("Hello.".to_string()).unwrap())
                .build();
            assert!(result.is_ok());
        }
        {
            let result = TweetBuilder::default()
                .user_id(UserId::try_from("taro1010".to_string()).unwrap())
                .build();
            assert!(result.is_err());
        }
    }
}
