use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchTweet;
use crate::model::tweet::Tweet;

pub struct FetchTweetUseCase {
    read_tweet: Box<dyn ReadTweet<UserId> + Sync + Send>,
}

#[async_trait]
impl FetchTweet for FetchTweetUseCase {
    async fn handle(&self, _tweet_id: &i64) -> Result<Tweet> {
        todo!()
    }
}
