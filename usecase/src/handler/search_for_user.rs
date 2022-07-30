use anyhow::Result;
use async_trait::async_trait;

use crate::handler::SearchForUser;
use crate::model::user::User;

pub struct SearchForUserUseCase {}

#[async_trait]
impl SearchForUser for SearchForUserUseCase {
    async fn handle(&self, _user_name: &str) -> Result<Option<User>> {
        todo!()
    }
}
