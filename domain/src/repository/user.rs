use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use crate::model::user::User;
use crate::model::user_id::UserId;

#[async_trait]
pub trait ReadUser<T> {
    async fn get(&self, value: T) -> Result<HashMap<UserId, User>>;
}

#[async_trait]
pub trait WriteUser {
    async fn create(&self, model: User) -> Result<UserId>;
}
