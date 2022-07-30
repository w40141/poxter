use anyhow::Result;
use async_trait::async_trait;

use crate::model::{
    tweet::Tweet,
    user::{RegisteredUser, User, UserWithProfile},
};

#[async_trait]
pub trait SignUp {
    async fn handler(&self, user: &RegisteredUser) -> Result<i64>;
}

#[async_trait]
pub trait SearchForUser {
    async fn handler(&self, user_name: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait FetchFollower {
    async fn handler(&self, user_id: &i64) -> Result<Vec<User>>;
}

#[async_trait]
pub trait FetchFollowee {
    async fn handler(&self, user_id: &i64) -> Result<Vec<User>>;
}

#[async_trait]
pub trait FetchTweet {
    async fn handler(&self, tweet_id: &i64) -> Result<Tweet>;
}

#[async_trait]
pub trait FetchAllTweets {
    async fn handler(&self, user_id: &i64) -> Result<Vec<Tweet>>;
}

#[async_trait]
pub trait FetchAllLikedTweets {
    async fn handler(&self, user_id: &i64) -> Result<Vec<Tweet>>;
}

#[async_trait]
pub trait FetchUsersLikedTweet {
    async fn handler(&self, tweet_id: &i64) -> Result<Vec<Tweet>>;
}

#[async_trait]
pub trait PostTweet {
    async fn handler(&self, user_id: &i64, tweet: &Tweet) -> Result<i64>;
}

#[async_trait]
pub trait ReplyTweet {
    async fn handler(&self, user_id: &i64, target_tweet_id: &i64, tweet: &Tweet) -> Result<i64>;
}

#[async_trait]
pub trait LikeTweet {
    async fn handler(&self, user_id: &i64, tweet_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait UnLikeTweet {
    async fn handler(&self, user_id: &i64, tweet_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait FollowUser {
    async fn handler(&self, user_id: &i64, target_user_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait UnFollowUser {
    async fn handler(&self, user_id: &i64, target_user_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait ShowProfile {
    async fn handler(&self, user_id: &i64) -> Result<UserWithProfile>;
}

#[async_trait]
pub trait UpdateProfile {
    async fn handler(&self, user_id: &i64, profile: &UserWithProfile) -> Result<i64>;
}
