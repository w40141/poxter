use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchAllLikedTweets;
use crate::model::tweet::Tweet;

#[derive()]
pub struct FetchAllLikedTweetsUseCase {}

#[async_trait]
impl FetchAllLikedTweets for FetchAllLikedTweetsUseCase {
    async fn handle(&self, _user_id: &i64) -> Result<Vec<Tweet>> {
        todo!()
    }
}
