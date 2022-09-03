use anyhow::{anyhow, Result};
use chrono::prelude::*;

use super::user_id::UserId;
use super::user_name::UserName;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    user_name: UserName,
    follower: Vec<UserId>,
    followee: Vec<UserId>,
    created_date: DateTime<Local>,
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn user_name(&self) -> &UserName {
        &self.user_name
    }

    pub fn follower(&self) -> &Vec<UserId> {
        &self.follower
    }

    pub fn followee(&self) -> &Vec<UserId> {
        &self.followee
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }
}

pub struct UserBuilder {
    id: UserId,
    user_name: Option<String>,
    follower: Vec<UserId>,
    followee: Vec<UserId>,
    created_date: DateTime<Local>,
}

impl UserBuilder {
    pub fn default() -> Self {
        Self {
            id: UserId::new(),
            user_name: None,
            follower: Vec::new(),
            followee: Vec::new(),
            created_date: Local::now(),
        }
    }
    pub fn user_name(mut self, v: String) -> Self {
        self.user_name = Some(v);
        self
    }

    pub fn follower(mut self, v: Vec<UserId>) -> Self {
        self.follower = v;
        self
    }

    pub fn followee(mut self, v: Vec<UserId>) -> Self {
        self.followee = v;
        self
    }

    pub fn build(&self) -> Result<User> {
        let user_name = match &self.user_name {
            Some(v) => UserName::try_from(v.clone())?,
            None => return Err(anyhow!("NotFound user_id.")),
        };
        let follower = self.follower.clone();
        let followee = self.followee.clone();

        Ok(User {
            id: self.id.clone(),
            user_name,
            follower,
            followee,
            created_date: self.created_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn user_test() {
        {
            // Correct
            let result = UserBuilder::default()
                .user_name("taro0123".to_string())
                .build();
            assert!(result.is_ok());
            let user = result.unwrap();
            assert!(user.follower().is_empty());
            assert!(user.followee().is_empty());
        }
        {
            // Correct
            let result = UserBuilder::default()
                .user_name("taro0123".to_string())
                .follower(vec![UserId::new(), UserId::new()])
                .followee(vec![UserId::new(), UserId::new(), UserId::new()])
                .build();
            assert!(result.is_ok());
            let user = result.unwrap();
            assert_eq!(user.follower().len(), 2);
            assert_eq!(user.followee().len(), 3);
        }
        {
            // Incorrect
            let result = UserBuilder::default().build();
            assert!(result.is_err());
        }
    }
}
