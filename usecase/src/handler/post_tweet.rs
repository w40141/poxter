use anyhow::Result;
use async_trait::async_trait;

use domain::model::tweet::Tweet;
use domain::repository::tweet::WriteTweet;

use crate::model::tweet::PostedTweet;

use super::PostTweet;

pub struct PostTweetUseCase {
    write_tweet: Box<dyn WriteTweet + Sync + Send>,
}

#[async_trait]
impl PostTweet<PostedTweet> for PostTweetUseCase {
    async fn handle(&self, model: PostedTweet) -> Result<()> {
        let tweet = Tweet::try_from(&model)?;
        self.write_tweet.post(tweet).await?;
        Ok(())
    }
}
