use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchUsersLikedTweet;
use crate::model::tweet::Tweet;

pub struct FetchUsersLikedTweetUseCase {}

#[async_trait]
impl FetchUsersLikedTweet for FetchUsersLikedTweetUseCase {
    async fn handle(&self, _tweet_id: &i64) -> Result<Vec<Tweet>> {
        todo!()
    }
}
