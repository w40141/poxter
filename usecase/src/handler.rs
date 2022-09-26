pub mod fetch_all_tweets;
pub mod post_tweet;
pub mod reply_tweet;
pub mod sing_up;

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use domain::model::{tweet::Tweet, tweet_id::TweetId, user_id::UserId};

#[async_trait]
pub trait PostTweet<T> {
    async fn handle(&self, model: T) -> Result<TweetId>;
}

#[async_trait]
pub trait ReplyTweet<T> {
    async fn handle(&self, model: T) -> Result<TweetId>;
}

#[async_trait]
pub trait FetchAllTweets<T> {
    async fn handle(&self, model: T) -> Result<HashMap<TweetId, Tweet>>;
}

#[async_trait]
pub trait SignUp<T> {
    async fn handle(&self, model: T) -> Result<UserId>;
}
