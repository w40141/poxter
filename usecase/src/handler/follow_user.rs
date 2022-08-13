use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FollowUser;

pub struct FollowUserUseCase {}

#[async_trait]
impl FollowUser for FollowUserUseCase {
    async fn handle(&self, _user_id: &i64, _target_user_idd: &i64) -> Result<i64> {
        todo!()
    }
}
