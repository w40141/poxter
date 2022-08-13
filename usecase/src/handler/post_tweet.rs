use anyhow::Result;
use async_trait::async_trait;

use crate::handler::PostTweet;
use crate::model::tweet::Tweet;

pub struct PostTweetUseCase {}

#[async_trait]
impl PostTweet for PostTweetUseCase {
    async fn handle(&self, _user_id: &i64, _tweet: &Tweet) -> Result<i64> {
        todo!()
    }
}
