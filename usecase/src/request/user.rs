use anyhow::{Error, Result};

use domain::model::user::{User, UserBuilder};

pub struct RegisteredUser {
    user_id: String,
    name: String,
    bio: Option<String>,
}

impl RegisteredUser {
    pub fn new(user_id: String, name: String, bio: Option<String>) -> Self {
        Self { user_id, name, bio }
    }
}

impl TryFrom<&RegisteredUser> for User {
    type Error = Error;
    fn try_from(v: &RegisteredUser) -> Result<Self, self::Error> {
        let m = UserBuilder::default()
            .user_id(v.user_id.clone())
            .name(v.name.clone());
        match &v.bio {
            Some(b) => m.bio(b.clone()).build(),
            None => m.build(),
        }
    }
}

pub struct UserWithProfile {}
