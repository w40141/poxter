use anyhow::Result;
use async_trait::async_trait;

use crate::handler::UpdateProfile;
use crate::model::user::UserWithProfile;

pub struct UpdateProfileUseCase {}

#[async_trait]
impl UpdateProfile for UpdateProfileUseCase {
    async fn handle(&self, _user_id: &i64, _profile: &UserWithProfile) -> Result<i64> {
        todo!()
    }
}
