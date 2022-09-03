use anyhow::{anyhow, Result};
use async_trait::async_trait;

use domain::model::user::User;
use domain::model::user_id::UserId;
use domain::repository::user::{ReadUser, WriteUser};

use crate::model::user::RegisteredUser;

use super::SignIn;

pub struct SignInUseCase {
    write_user: Box<dyn WriteUser + Sync + Send>,
    read_user: Box<dyn ReadUser<UserId> + Sync + Send>,
}

#[async_trait]
impl SignIn<RegisteredUser> for SignInUseCase {
    async fn handle(&self, model: RegisteredUser) -> Result<()> {
        let user = User::try_from(&model)?;
        let result = self.read_user.get(user.user_id().clone()).await?;
        if result.is_empty() {
            self.write_user.register(user).await?;
            Ok(())
        } else {
            Err(anyhow!("User is exist."))
        }
    }
}
