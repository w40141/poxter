use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

use crate::model::{reply_tweet::ReplyTweetRelation, tweet::Tweet};

#[async_trait]
pub trait ReadTweet<T> {
    async fn get(&self, model: T) -> Result<HashMap<Ulid, Tweet>>;
}

#[async_trait]
pub trait WriteTweet {
    async fn post(&self, tweet: Tweet) -> Result<Ulid>;
    async fn reply(&self, reply_tweet: ReplyTweetRelation) -> Result<Ulid>;
}
