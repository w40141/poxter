use ulid::Ulid;

pub struct ReplyTweetRelation {
    parent_id: Ulid,
    child_id: Ulid,
}

impl ReplyTweetRelation {
    pub fn new(parent_id: Ulid, child_id: Ulid) -> Self {
        Self {
            parent_id,
            child_id,
        }
    }

    pub fn parent_id(&self) -> &Ulid {
        &self.parent_id
    }

    pub fn child_id(&self) -> &Ulid {
        &self.child_id
    }
}
