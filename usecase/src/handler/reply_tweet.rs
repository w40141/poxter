use anyhow::Result;
use async_trait::async_trait;

use domain::model::reply_tweet::ReplyTweetRelation;
use domain::model::tweet::Tweet;
use domain::repository::tweet::WriteTweet;

use crate::handler::ReplyTweet;
use crate::model::tweet::RepliedTweet;

pub struct ReplyTweetUseCase {
    write_tweet: Box<dyn WriteTweet + Sync + Send>,
}

#[async_trait]
impl ReplyTweet<RepliedTweet> for ReplyTweetUseCase {
    async fn handle(&self, model: RepliedTweet) -> Result<()> {
        let tweet = Tweet::try_from(model.posted_tweet())?;
        let parent_id = self.write_tweet.post(tweet).await?;
        let reply_tweet = ReplyTweetRelation::new(parent_id, *model.target_tweet_id());
        self.write_tweet.reply(reply_tweet).await?;
        Ok(())
    }
}
