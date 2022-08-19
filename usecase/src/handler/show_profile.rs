use anyhow::Result;
use async_trait::async_trait;

use crate::handler::ShowProfile;
use crate::model::user::UserWithProfile;

pub struct ShowProfileUseCase {}

#[async_trait]
impl ShowProfile for ShowProfileUseCase {
    async fn handle(&self, _user_id: &i64) -> Result<UserWithProfile> {
        todo!()
    }
}
