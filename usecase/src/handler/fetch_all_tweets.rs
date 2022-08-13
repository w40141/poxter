use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchAllTweets;
use crate::model::tweet::Tweet;

#[derive()]
pub struct FetchAllTweetsUseCase {}

#[async_trait]
impl FetchAllTweets for FetchAllTweetsUseCase {
    async fn handle(&self, _user_id: &i64) -> Result<Vec<Tweet>> {
        todo!()
    }
}
