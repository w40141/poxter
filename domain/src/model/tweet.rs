use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;
use ulid::Ulid;
use unicode_segmentation::UnicodeSegmentation;

use crate::model::user_id::UserId;

#[derive(Debug, Clone)]
pub struct Tweet {
    id: Ulid,
    user_id: UserId,
    content: Content,
    created_date: DateTime<Local>,
}

impl Tweet {
    pub fn id(&self) -> &Ulid {
        &self.id
    }

    pub fn user_id(&self) -> &String {
        self.user_id.get()
    }

    pub fn content(&self) -> &String {
        self.content.get()
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }
}

#[derive(Debug, Clone)]
pub struct TweetBuilder {
    id: Ulid,
    user_id: Option<String>,
    content: Option<String>,
    created_date: DateTime<Local>,
}

impl TweetBuilder {
    pub fn default() -> Self {
        Self {
            id: Ulid::new(),
            user_id: None,
            content: None,
            created_date: Local::now(),
        }
    }

    pub fn user_id(mut self, v: String) -> Self {
        self.user_id = Some(v);
        self
    }

    pub fn content(mut self, v: String) -> Self {
        self.content = Some(v);
        self
    }

    pub fn build(&self) -> Result<Tweet> {
        let user_id = match &self.user_id {
            Some(v) => UserId::try_from(v.clone())?,
            None => return Err(anyhow!("NotFound user_id.")),
        };

        let content = match &self.content {
            Some(v) => Content::try_from(v.clone())?,
            None => return Err(anyhow!("NotFound content.")),
        };

        Ok(Tweet {
            id: self.id,
            user_id,
            content,
            created_date: self.created_date,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Content(String);

impl Content {
    pub fn get(&self) -> &String {
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

    #[test]
    fn tweet_test() {
        {
            let result = TweetBuilder::default()
                .user_id("taro1010".to_string())
                .content("Hello.".to_string())
                .build();
            assert!(result.is_ok());
        }
        {
            let result = TweetBuilder::default()
                .user_id("taro1010".to_string())
                .build();
            assert!(result.is_err());
        }
    }
}
