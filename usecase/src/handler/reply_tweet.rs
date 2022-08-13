use anyhow::Result;
use async_trait::async_trait;

use crate::handler::ReplyTweet;
use crate::model::tweet::Tweet;

pub struct ReplyTweetUseCase {}

#[async_trait]
impl ReplyTweet for ReplyTweetUseCase {
    async fn handle(&self, _user_id: &i64, _target_tweet_id: &i64, _tweet: &Tweet) -> Result<i64> {
        todo!()
    }
}
