use ulid::Ulid;

use super::id::Id;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TweetId(Id);

impl TweetId {
    pub fn new() -> Self {
        Self(Id::new())
    }

    pub fn user_id(&self) -> &Id {
        &self.0
    }
}

impl From<Id> for TweetId {
    fn from(v: Id) -> Self {
        TweetId(v)
    }
}

impl From<Ulid> for TweetId {
    fn from(v: Ulid) -> Self {
        TweetId(Id::from(v))
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn tweet_id_test() {
        let old_id = TweetId::new();
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        let new_id = TweetId::new();
        assert!(old_id < new_id);
    }
}
