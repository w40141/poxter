pub mod post;
pub mod user;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid user id")]
    InvalidUserId,
}
