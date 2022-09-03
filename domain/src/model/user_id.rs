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
