// pub mod fetch_all_liked_tweets;
// pub mod fetch_all_tweets;
// pub mod fetch_followee;
// pub mod fetch_follower;
// pub mod fetch_tweet;
// pub mod fetch_users_liked_tweet;
// pub mod follow_user;
// pub mod like_tweet;
pub mod post_tweet;
pub mod reply_tweet;
// pub mod search_for_user;
// pub mod show_profile;
// pub mod sing_up;
// pub mod un_follow_user;
// pub mod un_like_tweet;
// pub mod update_profile;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PostTweet<T> {
    async fn handle(&self, model: T) -> Result<()>;
}

#[async_trait]
pub trait ReplyTweet<T> {
    async fn handle(&self, model: T) -> Result<()>;
}

// #[async_trait]
// pub trait FetchAllLikedTweets<T> {
//     async fn handle(&self, model: T) -> Result<Vec<Tweet>>;
// }

// #[async_trait]
// pub trait FetchAllTweets {
//     async fn handle(&self, model: T) -> Result<Vec<Tweet>>;
// }

// #[async_trait]
// pub trait FetchFollowee {
//     async fn handle(&self, model: T) -> Result<Vec<User>>;
// }

// #[async_trait]
// pub trait FetchFollower {
//     async fn handle(&self, model: T) -> Result<Vec<User>>;
// }

// #[async_trait]
// pub trait FetchTweet {
//     async fn handle(&self, model: T) -> Result<Tweet>;
// }

// #[async_trait]
// pub trait FetchUsersLikedTweet {
//     async fn handle(&self, model: T) -> Result<Vec<Tweet>>;
// }

// #[async_trait]
// pub trait FollowUser<T> {
//     async fn handle(&self, model: T) -> Result<i64>;
// }

// #[async_trait]
// pub trait LikeTweet {
//     async fn handle(&self, user_id: &i64, tweet_id: &i64) -> Result<i64>;
// }

// #[async_trait]
// pub trait SearchForUser {
//     async fn handle(&self, user_name: &str) -> Result<Option<User>>;
// }

// #[async_trait]
// pub trait ShowProfile {
//     async fn handle(&self, user_id: &i64) -> Result<UserWithProfile>;
// }

// #[async_trait]
// pub trait SignUp {
//     async fn handle(&self, user: &RegisteredUser) -> Result<i64>;
// }

// #[async_trait]
// pub trait UnFollowUser {
//     async fn handle(&self, user_id: &i64, target_user_id: &i64) -> Result<i64>;
// }

// #[async_trait]
// pub trait UnLikeTweet {
//     async fn handle(&self, user_id: &i64, tweet_id: &i64) -> Result<i64>;
// }

// #[async_trait]
// pub trait UpdateProfile {
//     async fn handle(&self, user_id: &i64, profile: &UserWithProfile) -> Result<i64>;
// }
