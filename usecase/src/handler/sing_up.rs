use anyhow::Result;
use async_trait::async_trait;

use crate::handler::SignUp;
use crate::model::user::RegisteredUser;

pub struct SingUpUseCase {}

#[async_trait]
impl SignUp for SingUpUseCase {
    async fn handle(&self, _user: &RegisteredUser) -> Result<i64> {
        todo!()
    }
}
