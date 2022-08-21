// use chrono::prelude::{DateTime, Local};
// use derive_builder::Builder;
// use validator::Validate;
//
// use super::user::UserId;
//
// #[derive(Debug, Clone, Builder)]
// #[builder(build_fn(validate = "Self::validate"))]
// pub struct Post {
//     user_id: UserId,
//     tweet: Tweet,
//     // TODO: Make it the correct type
//     created_date: DateTime<Local>,
// }
//
// impl Post {
//     pub fn user_id(&self) -> &String {
//         &self.user_id.value()
//     }
//
//     pub fn tweet(&self) -> &String {
//         &self.tweet.value()
//     }
//
//     pub fn created_date(&self) -> &DateTime<Local> {
//         &self.created_date
//     }
// }
//
// impl PostBuilder {
//     fn validate(&self) -> Result<(), String> {
//         if let Some(ref value) = self.tweet {
//             match value.validate() {
//                 Ok(_) => Ok(()),
//                 Err(_) => Err("Tweet must be 200 characters or less.".to_string()),
//             }
//         } else {
//             Ok(())
//         }
//     }
// }
//
// #[derive(Debug, Clone, Validate)]
// pub struct Tweet {
//     #[validate(length(min = 1, max = 200))]
//     value: String,
// }
//
// impl Tweet {
//     pub fn value(&self) -> &String {
//         &self.value
//     }
// }
//
// impl From<String> for Tweet {
//     fn from(value: String) -> Self {
//         Tweet { value }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn tweet_test() {
//         {
//             // Correct
//             let tweet = Tweet {
//                 value:
//                     "The Adventures of Tom Sawyer is set in the 1840's in the fictitious town of St.
// Petersburg, Missouri, where Tom lives with his deceased mother's sister,
// Aunt Polly, and his half-brother, Sid. After To"
//                         .to_string(),
//             };
//             let result = tweet.validate();
//             assert_eq!(result.is_ok(), true)
//         }
//         {
//             // Correct
//             let tweet = Tweet {
//                 value: "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしす。"
//                     .to_string(),
//             };
//             let result = tweet.validate();
//             assert_eq!(result.is_ok(), true)
//         }
//         {
//             // Incorrect because value is longer than 201 characters.
//             let user_id = Tweet {
//                 value:
//                     "The Adventures of Tom Sawyer is set in the 1840's in the fictitious town of St.
// Petersburg, Missouri, where Tom lives with his deceased mother's sister,
// Aunt Polly, and his half-brother, Sid. After Too"
//                         .to_string(),
//             };
//             let result = user_id.validate();
//             assert_eq!(result.is_err(), true)
//         }
//         {
//             // Incorrect because value is longer than 201 characters.
//             let tweet = Tweet {
//                 value: "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへ。
// あいうえおかきくけこさしす。あ"
//                     .to_string(),
//             };
//             let result = tweet.validate();
//             assert_eq!(result.is_err(), true)
//         }
//     }
//
//     #[test]
//     fn post_test() {
//         {
//             let now_time = chrono::Local::now();
//             let result = PostBuilder::default()
//                 .user_id(UserId::from("taro".to_string()))
//                 .tweet(Tweet::from("Hello.".to_string()))
//                 .created_date(now_time)
//                 .build();
//             assert_eq!(result.is_ok(), true);
//         }
//         {
//             let now_time = chrono::Local::now();
//             let tweet =
//                 "The Adventures of Tom Sawyer is set in the 1840's in the fictitious town of St.
// Petersburg, Missouri, where Tom lives with his deceased mother's sister,
// Aunt Polly, and his half-brother, Sid. After Too"
//                     .to_string();
//             let result = PostBuilder::default()
//                 .user_id(UserId::from("taro".to_string()))
//                 .tweet(Tweet::from(tweet))
//                 .created_date(now_time)
//                 .build();
//             assert_eq!(result.is_err(), true);
//         }
//     }
// }
