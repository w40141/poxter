use anyhow::Result;
use async_trait::async_trait;

use crate::model::{user::User, user_id::UserId};

#[async_trait]
pub trait ReadUser<T> {
    async fn get(&self, require: T) -> Result<User>;
}

#[async_trait]
pub trait WriteUser<T> {
    async fn register(&self, model: T) -> Result<UserId>;
}
