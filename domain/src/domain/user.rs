use anyhow::{anyhow, Result};

use crate::domain::user_id::UserId;

#[derive(Debug, Clone)]
pub struct User {
    user_id: UserId,
    name: String,
    bio: Option<String>,
    follower: i64,
    followee: i64,
}

impl User {
    pub fn user_id(&self) -> &String {
        self.user_id.get()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }

    pub fn follower(&self) -> &i64 {
        &self.follower
    }

    pub fn followee(&self) -> &i64 {
        &self.followee
    }
}

pub struct UserBuilder {
    user_id: Option<UserId>,
    name: Option<String>,
    bio: Option<String>,
    follower: i64,
    followee: i64,
}

impl UserBuilder {
    pub fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            bio: None,
            follower: 0,
            followee: 0,
        }
    }
    pub fn user_id(mut self, v: UserId) -> Self {
        self.user_id = Some(v);
        self
    }

    pub fn name(mut self, v: String) -> Self {
        self.name = Some(v);
        self
    }

    pub fn bio(mut self, v: String) -> Self {
        self.bio = Some(v);
        self
    }

    pub fn follower(mut self, v: i64) -> Self {
        self.follower = v;
        self
    }

    pub fn followee(mut self, v: i64) -> Self {
        self.followee = v;
        self
    }

    pub fn build(&self) -> Result<User> {
        let user_id = match &self.user_id {
            Some(v) => v.clone(),
            None => return Err(anyhow!("NotFound user_id.")),
        };
        let name = match &self.name {
            Some(v) => v.clone(),
            None => return Err(anyhow!("NotFound name.")),
        };
        let bio = self.bio.clone();
        let follower = self.follower;
        let followee = self.followee;

        Ok(User {
            user_id,
            name,
            bio,
            follower,
            followee,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_test() {
        {
            // Correct
            let result = UserBuilder::default()
                .name("taro".to_string())
                .user_id(UserId::try_from("taro0123".to_string()).unwrap())
                .bio("Hello!".to_string())
                .follower(10)
                .followee(20)
                .build();
            assert!(result.is_ok());
            let user = result.unwrap();
            assert_eq!(user.name(), "taro");
            assert_eq!(user.user_id(), &"taro0123");
            assert_eq!(user.bio(), &Some("Hello!".to_string()));
            assert_eq!(user.follower(), &10);
            assert_eq!(user.followee(), &20);
        }
        {
            // Correct
            let result = UserBuilder::default()
                .name("taro".to_string())
                .user_id(UserId::try_from("taro0123".to_string()).unwrap())
                .build();
            assert!(result.is_ok());
            let user = result.unwrap();
            assert_eq!(user.name(), "taro");
            assert_eq!(user.user_id(), &"taro0123");
            assert_eq!(user.bio(), &None);
            assert_eq!(user.follower(), &0);
            assert_eq!(user.followee(), &0);
        }
        {
            // Incorrect because user id is invalid.
            let result = UserBuilder::default().name("taro".to_string()).build();
            assert!(result.is_err());
        }
    }
}
