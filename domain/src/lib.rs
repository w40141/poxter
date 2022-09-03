pub mod model;

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

use crate::model::user::User;
use crate::model::{reply_tweet::ReplyTweetRelation, tweet::Tweet};

#[async_trait]
pub trait ReadTweet<T> {
    async fn get(&self, model: T) -> Result<HashMap<Ulid, Tweet>>;
}

#[async_trait]
pub trait WriteTweet {
    async fn post(&self, model: Tweet) -> Result<Ulid>;
    async fn reply(&self, model: ReplyTweetRelation) -> Result<Ulid>;
}

#[async_trait]
pub trait ReadUser<T> {
    async fn get(&self, model: T) -> Result<HashMap<Ulid, User>>;
}

#[async_trait]
pub trait WriteUser {
    async fn register(&self, model: User) -> Result<Ulid>;
}
