use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

use crate::domain::tweet::Tweet;

#[async_trait]
pub trait ReadTweet {
    async fn get(&self) -> Result<HashMap<Ulid, Tweet>>;
}

#[async_trait]
pub trait WriteTweet {
    async fn post(&self) -> Result<Ulid>;
    async fn reply(&self) -> Result<Ulid>;
}
