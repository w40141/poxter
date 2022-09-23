use std::cmp::Ordering;

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
    pub fn new(user_id: UserId, content: Content) -> Self {
        Self {
            tweet_id: TweetId::new(),
            user_id,
            content,
            created_date: Local::now(),
        }
    }

    pub fn tweet_id(&self) -> &TweetId {
        &self.tweet_id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn content(&self) -> &String {
        self.content.get()
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let r = if self.tweet_id > other.tweet_id {
            Ordering::Greater
        } else if self.tweet_id == other.tweet_id {
            Ordering::Equal
        } else {
            Ordering::Less
        };
        Some(r)
    }
}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.tweet_id == other.tweet_id
    }
}

pub struct TweetBuilder {
    tweet_id: Option<TweetId>,
    user_id: Option<UserId>,
    content: Option<String>,
    created_date: Option<DateTime<Local>>,
}

impl TweetBuilder {
    pub fn default() -> Self {
        Self {
            tweet_id: None,
            user_id: None,
            content: None,
            created_date: None,
        }
    }

    pub fn tweet_id(mut self, v: TweetId) -> Self {
        self.tweet_id = Some(v);
        self
    }

    pub fn user_id(mut self, v: UserId) -> Self {
        self.user_id = Some(v);
        self
    }

    pub fn content(mut self, v: String) -> Self {
        self.content = Some(v);
        self
    }

    pub fn created_date(mut self, v: DateTime<Local>) -> Self {
        self.created_date = Some(v);
        self
    }

    pub fn build(self) -> Result<Tweet> {
        let tweet_id = match self.tweet_id {
            Some(v) => v,
            None => return Err(anyhow!("NotFound id.")),
        };

        let user_id = match self.user_id {
            Some(v) => v,
            None => return Err(anyhow!("NotFound user_id.")),
        };

        let content = match self.content {
            Some(v) => Content::try_from(v)?,
            None => return Err(anyhow!("NotFound user_id.")),
        };

        let created_date = match self.created_date {
            Some(v) => v,
            None => Local::now(),
        };

        Ok(Tweet {
            tweet_id,
            user_id,
            content,
            created_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn tweet_test() {
        let content = "abc".to_string();

        {
            // Correct
            let result = TweetBuilder::default()
                .tweet_id(TweetId::new())
                .user_id(UserId::new())
                .content(content.clone())
                .build();
            assert!(result.is_ok());
        }
        {
            // Incorrect
            let result = TweetBuilder::default()
                .user_id(UserId::new())
                .content(content.clone())
                .build();
            assert!(result.is_err());
        }
        {
            // Incorrect
            let result = TweetBuilder::default()
                .tweet_id(TweetId::new())
                .content(content)
                .build();
            assert!(result.is_err());
        }
        {
            // Incorrect
            let result = TweetBuilder::default()
                .tweet_id(TweetId::new())
                .user_id(UserId::new())
                .build();
            assert!(result.is_err());
        }
    }

    #[test]
    fn tweet_order_test() {
        let content = "abc".to_string();

        let old_tweet = TweetBuilder::default()
            .tweet_id(TweetId::new())
            .user_id(UserId::new())
            .content(content.clone())
            .build()
            .unwrap();
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        let new_tweet = TweetBuilder::default()
            .tweet_id(TweetId::new())
            .user_id(UserId::new())
            .content(content)
            .build()
            .unwrap();
        assert!(old_tweet < new_tweet);
    }
}
