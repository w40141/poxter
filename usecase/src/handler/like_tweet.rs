use anyhow::Result;
use async_trait::async_trait;

use crate::handler::LikeTweet;

pub struct LikeTweetUseCase {}

#[async_trait]
impl LikeTweet for LikeTweetUseCase {
    async fn handle(&self, _user_id: &i64, _tweet_idd: &i64) -> Result<i64> {
        todo!()
    }
}
