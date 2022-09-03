use anyhow::{Error, Result};
use ulid::Ulid;

use domain::model::tweet::{Tweet, TweetBuilder};

pub struct PostedTweet {
    user_id: String,
    tweet: String,
}

impl PostedTweet {
    pub fn new(user_id: String, tweet: String) -> Self {
        Self { user_id, tweet }
    }
}

impl TryFrom<&PostedTweet> for Tweet {
    type Error = Error;
    fn try_from(v: &PostedTweet) -> Result<Self, self::Error> {
        TweetBuilder::default()
            .user_id(v.user_id.clone())
            .content(v.tweet.clone())
            .build()
    }
}

pub struct RepliedTweet {
    posted_tweet: PostedTweet,
    target_tweet_id: Ulid,
}

impl RepliedTweet {
    pub fn new(user_id: String, tweet: String, target_tweet_id: Ulid) -> Self {
        Self {
            posted_tweet: PostedTweet { user_id, tweet },
            target_tweet_id,
        }
    }

    pub fn posted_tweet(&self) -> &PostedTweet {
        &self.posted_tweet
    }

    pub fn target_tweet_id(&self) -> &Ulid {
        &self.target_tweet_id
    }
}
