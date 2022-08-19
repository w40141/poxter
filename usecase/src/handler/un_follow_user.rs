use anyhow::Result;
use async_trait::async_trait;

use crate::handler::UnFollowUser;

pub struct UnFollowUserUseCase {}

#[async_trait]
impl UnFollowUser for UnFollowUserUseCase {
    async fn handle(&self, _user_id: &i64, _target_user_id: &i64) -> Result<i64> {
        todo!()
    }
}
