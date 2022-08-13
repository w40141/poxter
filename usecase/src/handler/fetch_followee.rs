use anyhow::Result;
use async_trait::async_trait;

use crate::handler::FetchFollowee;
use crate::model::user::User;

#[derive()]
pub struct FetchFolloweeUseCase {}

#[async_trait]
impl FetchFollowee for FetchFolloweeUseCase {
    async fn handle(&self, _user_id: &i64) -> Result<Vec<User>> {
        todo!()
    }
}
