use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use ulid::Ulid;

use super::bio::Bio;
use super::user_id::UserId;
use super::user_name::UserName;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    user_name: UserName,
    follower: Vec<UserId>,
    followee: Vec<UserId>,
    bio: Bio,
    created_date: DateTime<Local>,
    updated_date: DateTime<Local>,
}

impl User {
    pub fn new(
        user_name: UserName,
        follower: Vec<UserId>,
        followee: Vec<UserId>,
        bio: Bio,
    ) -> Self {
        Self {
            id: UserId::new(),
            user_name,
            follower,
            followee,
            bio,
            created_date: Local::now(),
            updated_date: Local::now(),
        }
    }

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
        let r = if self.id > other.id {
            Ordering::Greater
        } else if self.id == other.id {
            Ordering::Equal
        } else {
            Ordering::Less
        };
        Some(r)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct UserBuilder {
    id: Option<Ulid>,
    user_name: Option<String>,
    follower: Vec<Ulid>,
    followee: Vec<Ulid>,
    bio: Option<Bio>,
    created_date: Option<DateTime<Local>>,
    updated_date: Option<DateTime<Local>>,
}

impl UserBuilder {
    pub fn default() -> Self {
        Self {
            id: None,
            user_name: None,
            follower: Vec::new(),
            followee: Vec::new(),
            bio: None,
            created_date: None,
            updated_date: None,
        }
    }

    pub fn id(mut self, v: Ulid) -> Self {
        self.id = Some(v);
        self
    }

    pub fn user_name(mut self, v: String) -> Self {
        self.user_name = Some(v);
        self
    }

    pub fn follower(mut self, v: Vec<Ulid>) -> Self {
        self.follower = v;
        self
    }

    pub fn followee(mut self, v: Vec<Ulid>) -> Self {
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
        let id = match self.id {
            Some(v) => UserId::from(v),
            None => return Err(anyhow!("NotFound id.")),
        };

        let user_name = match self.user_name {
            Some(v) => UserName::try_from(v)?,
            None => return Err(anyhow!("NotFound user name.")),
        };

        let follower = self.follower.iter().map(|v| UserId::from(*v)).collect();

        let followee = self.followee.iter().map(|v| UserId::from(*v)).collect();

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
            id,
            user_name,
            follower,
            followee,
            bio,
            created_date,
            updated_date,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
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
                .id(Ulid::new())
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
                .id(Ulid::new())
                .user_name("taro0123".to_string())
                .bio(bio.clone())
                .follower(vec![Ulid::new(), Ulid::new()])
                .followee(vec![Ulid::new(), Ulid::new(), Ulid::new()])
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

        let old_user = User::new(user_name.clone(), vec![], vec![], bio.clone());
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        let new_user = User::new(user_name, vec![], vec![], bio.clone());
        assert!(old_user < new_user);
    }
}
