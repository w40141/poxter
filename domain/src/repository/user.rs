use anyhow::Result;
use async_trait::async_trait;

use crate::domain::{user::User, user_id::UserId};

#[async_trait]
pub trait ReadUser {
    async fn get(&self) -> Result<User>;
}

#[async_trait]
pub trait WriteUser {
    async fn register(&self) -> Result<UserId>;
}
