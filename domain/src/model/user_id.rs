use ulid::Ulid;

use super::id::Id;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UserId(Id);

impl UserId {
    pub fn new() -> Self {
        Self(Id::new())
    }

    pub fn user_id(&self) -> &Id {
        &self.0
    }
}

impl From<Id> for UserId {
    fn from(v: Id) -> Self {
        UserId(v)
    }
}

impl From<Ulid> for UserId {
    fn from(v: Ulid) -> Self {
        UserId(Id::from(v))
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn user_id_test() {
        let old_id = UserId::new();
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        let new_id = UserId::new();
        assert!(old_id < new_id);
    }
}
