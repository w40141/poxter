use super::tweet_id::TweetId;

#[derive(Debug, Clone)]
pub struct ReplyTweetRelation {
    parent_id: TweetId,
    child_id: TweetId,
}

impl ReplyTweetRelation {
    pub fn new(parent_id: TweetId, child_id: TweetId) -> Self {
        Self {
            parent_id,
            child_id,
        }
    }

    pub fn parent_id(&self) -> &TweetId {
        &self.parent_id
    }

    pub fn child_id(&self) -> &TweetId {
        &self.child_id
    }
}
