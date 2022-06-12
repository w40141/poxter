use anyhow::Result;
use derive_builder::Builder;
use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct User {
    name: String,
    user_id: UserId,
    #[builder(default)]
    bio: Option<String>,
    #[builder(default = "0")]
    follower: i64,
    #[builder(default = "0")]
    followee: i64,
}

impl User {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn user_id(&self) -> &String {
        &self.user_id.value()
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

impl UserBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref value) = self.user_id {
            match value.validate() {
                Ok(_) => Ok(()),
                Err(_) => Err("Invalid user id".to_string()),
            }
        } else {
            Ok(())
        }
    }
}

// [[:word:]] = [0-9A-Za-z_]
static WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[[:word:]]*$").unwrap());

#[derive(Debug, Clone, Validate)]
pub struct UserId {
    #[validate(length(min = 5, max = 32), regex = "WORD")]
    value: String,
}

impl UserId {
    pub fn value(&self) -> &String {
        &self.value
    }
}

impl From<String> for UserId {
    fn from(value: String) -> Self {
        UserId { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_test() {
        {
            // Correct
            let user_id = UserId {
                value: "adf130_".to_string(),
            };
            let result = user_id.validate();
            assert_eq!(result.is_ok(), true)
        }
        {
            // Incorrect because value is less than 5 characters.
            let user_id = UserId {
                value: "adf1".to_string(),
            };
            let result = user_id.validate();
            assert_eq!(result.is_err(), true)
        }
        {
            // Incorrect because value has the characters out WORD.
            let user_id = UserId {
                value: "adf130_-*".to_string(),
            };
            let result = user_id.validate();
            assert_eq!(result.is_err(), true)
        }
    }

    #[test]
    fn user_test() {
        {
            // Correct
            let result = UserBuilder::default()
                .name("taro".to_string())
                .user_id(UserId::from("taro0123".to_string()))
                .bio(Some("Hello!".to_string()))
                .follower(10)
                .followee(20)
                .build();
            assert_eq!(result.is_ok(), true);
            let user = result.unwrap();
            assert_eq!(user.name, "taro");
            assert_eq!(user.user_id.value, "taro0123");
            assert_eq!(user.bio, Some("Hello!".to_string()));
            assert_eq!(user.follower, 10);
            assert_eq!(user.followee, 20);
        }
        {
            // Correct
            let result = UserBuilder::default()
                .name("taro".to_string())
                .user_id(UserId::from("taro0123".to_string()))
                .build();
            assert_eq!(result.is_ok(), true);
            let user = result.unwrap();
            assert_eq!(user.name, "taro");
            assert_eq!(user.user_id.value, "taro0123");
            assert_eq!(user.bio, None);
            assert_eq!(user.follower, 0);
            assert_eq!(user.followee, 0);
        }
        {
            // Incorrect because user id is invalid.
            let result = UserBuilder::default()
                .name("taro".to_string())
                .user_id(UserId::from("taro0123-".to_string()))
                .build();
            assert_eq!(result.is_err(), true);
        }
    }
}
