use anyhow::{anyhow, Result};
use chrono::prelude::*;

use super::content::Content;
use super::tweet_id::TweetId;
use super::user_id::UserId;

#[derive(Debug, Clone)]
pub struct Tweet {
    tweet_id: TweetId,
    user_id: UserId,
    content: Content,
    created_date: DateTime<Local>,
}

impl Tweet {
    pub fn tweet_id(&self) -> &TweetId {
        &self.tweet_id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn content(&self) -> &String {
        self.content.content()
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }
}

pub struct TweetBuilder {
    tweet_id: TweetId,
    user_id: Option<UserId>,
    content: Option<String>,
    created_date: DateTime<Local>,
}

impl TweetBuilder {
    pub fn default() -> Self {
        Self {
            tweet_id: TweetId::new(),
            user_id: None,
            content: None,
            created_date: Local::now(),
        }
    }
    pub fn user_id(mut self, v: UserId) -> Self {
        self.user_id = Some(v);
        self
    }

    pub fn content(mut self, v: String) -> Self {
        self.content = Some(v);
        self
    }

    pub fn build(&self) -> Result<Tweet> {
        let user_id = match &self.user_id {
            Some(v) => v.clone(),
            None => return Err(anyhow!("NotFound user_id.")),
        };
        let content = match &self.content {
            Some(v) => Content::try_from(v.clone())?,
            None => return Err(anyhow!("NotFound user_id.")),
        };

        Ok(Tweet {
            tweet_id: self.tweet_id.clone(),
            user_id,
            content,
            created_date: self.created_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tweet_test() {
        {
            // Correct
            let result = TweetBuilder::default()
                .user_id(UserId::new())
                .content("abc".to_string())
                .build();
            assert!(result.is_ok());
        }
        {
            // Incorrect
            let result = TweetBuilder::default().build();
            assert!(result.is_err());
        }
        {
            // Incorrect
            let result = TweetBuilder::default().user_id(UserId::new()).build();
            assert!(result.is_err());
        }
        {
            // Incorrect
            let result = TweetBuilder::default().content("abc".to_string()).build();
            assert!(result.is_err());
        }
    }
}
