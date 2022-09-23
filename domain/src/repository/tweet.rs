use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

use crate::model::tweet::Tweet;

#[async_trait]
pub trait ReadTweet<T> {
    async fn get(&self, model: T) -> Result<HashMap<Ulid, Tweet>>;
}

#[async_trait]
pub trait WriteTweet {
    async fn create(&self, model: Tweet) -> Result<Ulid>;
}
