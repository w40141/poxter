use anyhow::Result;
use async_trait::async_trait;

use crate::handler::UnLikeTweet;

pub struct UnLikeTweetUseCase {}

#[async_trait]
impl UnLikeTweet for UnLikeTweetUseCase {
    async fn handle(&self, _user_id: &i64, _target_user_id: &i64) -> Result<i64> {
        todo!()
    }
}
