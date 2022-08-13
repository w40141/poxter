use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchTweet;
use crate::model::tweet::Tweet;

pub struct FetchTweetUseCase {}

#[async_trait]
impl FetchTweet for FetchTweetUseCase {
    async fn handle(&self, _tweet_id: &i64) -> Result<Tweet> {
        todo!()
    }
}
