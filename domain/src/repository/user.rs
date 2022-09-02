use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

use crate::model::{user::User, user_id::UserId};

#[async_trait]
pub trait ReadUser<T> {
    async fn get(&self, model: T) -> Result<HashMap<Ulid, User>>;
}

#[async_trait]
pub trait WriteUser {
    async fn register(&self, model: User) -> Result<UserId>;
}
