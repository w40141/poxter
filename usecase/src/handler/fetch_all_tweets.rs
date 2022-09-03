use anyhow::{Ok, Result};
use async_trait::async_trait;
use ulid::Ulid;

use domain::model::tweet::Tweet;
use domain::model::user_id::UserId;
use domain::repository::tweet::ReadTweet;

use super::FetchAllTweets;

#[derive()]
pub struct FetchAllTweetsUseCase {
    read_tweet: Box<dyn ReadTweet<UserId> + Sync + Send>,
}

#[async_trait]
impl FetchAllTweets<UserId> for FetchAllTweetsUseCase {
    async fn handle(&self, model: UserId) -> Result<Vec<(Ulid, Tweet)>> {
        let map = self.read_tweet.get(model).await?;
        let mut vec: Vec<_> = map.iter().map(|(x, y)| (x.clone(), y.clone())).collect();
        vec.sort_by(|x, y| (x.0).cmp(&y.0));
        Ok(vec)
    }
}
