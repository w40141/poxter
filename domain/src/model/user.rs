use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use chrono::prelude::*;

use super::bio::Bio;
use super::user_id::UserId;
use super::user_name::UserName;

#[derive(Debug, Clone)]
pub struct User {
    user_id: UserId,
    user_name: UserName,
    follower: Vec<UserId>,
    followee: Vec<UserId>,
    bio: Bio,
    created_date: DateTime<Local>,
    updated_date: DateTime<Local>,
}

impl User {
    pub fn new(user_name: UserName, bio: Bio) -> Self {
        Self {
            user_id: UserId::new(),
            user_name,
            follower: vec![],
            followee: vec![],
            bio,
            created_date: Local::now(),
            updated_date: Local::now(),
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
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

    pub fn bio(&self) -> &Bio {
        &self.bio
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }

    pub fn updated_date(&self) -> &DateTime<Local> {
        &self.updated_date
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let r = if self.user_id > other.user_id {
            Ordering::Greater
        } else if self.user_id == other.user_id {
            Ordering::Equal
        } else {
            Ordering::Less
        };
        Some(r)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

pub struct UserBuilder {
    user_id: Option<UserId>,
    user_name: Option<String>,
    follower: Vec<UserId>,
    followee: Vec<UserId>,
    bio: Option<Bio>,
    created_date: Option<DateTime<Local>>,
    updated_date: Option<DateTime<Local>>,
}

impl UserBuilder {
    pub fn default() -> Self {
        Self {
            user_id: None,
            user_name: None,
            follower: vec![],
            followee: vec![],
            bio: None,
            created_date: None,
            updated_date: None,
        }
    }

    pub fn user_id(mut self, v: UserId) -> Self {
        self.user_id = Some(v);
        self
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

    pub fn bio(mut self, v: Bio) -> Self {
        self.bio = Some(v);
        self
    }

    pub fn created_date(mut self, v: DateTime<Local>) -> Self {
        self.created_date = Some(v);
        self
    }

    pub fn updated_date(mut self, v: DateTime<Local>) -> Self {
        self.updated_date = Some(v);
        self
    }

    pub fn build(self) -> Result<User> {
        let user_id = match self.user_id {
            Some(v) => v,
            None => UserId::new(),
        };

        let user_name = match self.user_name {
            Some(v) => UserName::try_from(v)?,
            None => return Err(anyhow!("NotFound user name.")),
        };

        let bio = match self.bio {
            Some(v) => v,
            None => return Err(anyhow!("NotFound bio.")),
        };

        let created_date = match self.created_date {
            Some(v) => v,
            None => Local::now(),
        };

        let updated_date = match self.updated_date {
            Some(v) => v,
            None => Local::now(),
        };

        Ok(User {
            user_id,
            user_name,
            follower: self.follower,
            followee: self.followee,
            bio,
            created_date,
            updated_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::super::bio::BioBuilder;

    use super::*;

    #[test]
    fn user_test() {
        let bio = &BioBuilder::default()
            .profit_name("taro".to_string())
            .content("abc".to_string())
            .build()
            .unwrap();
        {
            // Correct
            let result = UserBuilder::default()
                .user_name("taro0123".to_string())
                .bio(bio.clone())
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
                .bio(bio.clone())
                .follower((0..2).map(|_| UserId::new()).collect())
                .followee((0..3).map(|_| UserId::new()).collect())
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

    #[test]
    fn user_order_test() {
        let bio = &BioBuilder::default()
            .profit_name("taro".to_string())
            .content("abc".to_string())
            .build()
            .unwrap();
        let user_name = UserName::try_from("taro0123".to_string()).unwrap();

        let old_user = User::new(user_name.clone(), bio.clone());
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        let new_user = User::new(user_name, bio.clone());
        assert!(old_user < new_user);
    }
}
