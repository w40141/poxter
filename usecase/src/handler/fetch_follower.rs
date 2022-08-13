use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchFollower;
use crate::model::user::User;

pub struct FetchFollowerUseCase {}

#[async_trait]
impl FetchFollower for FetchFollowerUseCase {
    async fn handle(&self, _user_id: &i64) -> Result<Vec<User>> {
        todo!()
    }
}
