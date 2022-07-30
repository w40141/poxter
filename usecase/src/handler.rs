pub mod fetch_all_liked_tweets;
pub mod fetch_all_tweets;
pub mod fetch_followee;
pub mod fetch_follower;
pub mod fetch_tweet;
pub mod follow_user;
pub mod like_tweet;
pub mod post_tweet;
pub mod reply_tweet;
pub mod search_for_user;
pub mod show_profile;
pub mod sing_up;
pub mod unfollow_user;
pub mod unlike_tweet;
pub mod update_profile;

use anyhow::Result;
use async_trait::async_trait;

use crate::model::{
    tweet::Tweet,
    user::{RegisteredUser, User, UserWithProfile},
};

#[async_trait]
pub trait SignUp {
    async fn handle(&self, user: &RegisteredUser) -> Result<i64>;
}

#[async_trait]
pub trait SearchForUser {
    async fn handle(&self, user_name: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait FetchFollower {
    async fn handle(&self, user_id: &i64) -> Result<Vec<User>>;
}

#[async_trait]
pub trait FetchFollowee {
    async fn handle(&self, user_id: &i64) -> Result<Vec<User>>;
}

#[async_trait]
pub trait FetchTweet {
    async fn handle(&self, tweet_id: &i64) -> Result<Tweet>;
}

#[async_trait]
pub trait FetchAllTweets {
    async fn handle(&self, user_id: &i64) -> Result<Vec<Tweet>>;
}

#[async_trait]
pub trait FetchAllLikedTweets {
    async fn handle(&self, user_id: &i64) -> Result<Vec<Tweet>>;
}

#[async_trait]
pub trait FetchUsersLikedTweet {
    async fn handle(&self, tweet_id: &i64) -> Result<Vec<Tweet>>;
}

#[async_trait]
pub trait PostTweet {
    async fn handle(&self, user_id: &i64, tweet: &Tweet) -> Result<i64>;
}

#[async_trait]
pub trait ReplyTweet {
    async fn handle(&self, user_id: &i64, target_tweet_id: &i64, tweet: &Tweet) -> Result<i64>;
}

#[async_trait]
pub trait LikeTweet {
    async fn handle(&self, user_id: &i64, tweet_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait UnLikeTweet {
    async fn handle(&self, user_id: &i64, tweet_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait FollowUser {
    async fn handle(&self, user_id: &i64, target_user_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait UnFollowUser {
    async fn handle(&self, user_id: &i64, target_user_id: &i64) -> Result<i64>;
}

#[async_trait]
pub trait ShowProfile {
    async fn handle(&self, user_id: &i64) -> Result<UserWithProfile>;
}

#[async_trait]
pub trait UpdateProfile {
    async fn handle(&self, user_id: &i64, profile: &UserWithProfile) -> Result<i64>;
}
