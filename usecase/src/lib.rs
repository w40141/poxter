pub mod fetch_all_tweets;
pub mod post_tweet;
pub mod reply_tweet;
pub mod sign_in;

use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

use domain::model::tweet::Tweet;

#[async_trait]
pub trait PostTweet<T> {
    async fn handle(&self, model: T) -> Result<()>;
}

#[async_trait]
pub trait ReplyTweet<T> {
    async fn handle(&self, model: T) -> Result<()>;
}

#[async_trait]
pub trait FetchAllTweets<T> {
    async fn handle(&self, model: T) -> Result<Vec<(Ulid, Tweet)>>;
}

#[async_trait]
pub trait SignIn<T> {
    async fn handle(&self, model: T) -> Result<()>;
}
