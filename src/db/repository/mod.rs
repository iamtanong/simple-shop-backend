use async_trait::async_trait;

use crate::{models, utils::error::AppError};

pub mod users;

#[async_trait]
pub trait UserRepo {
    async fn create_user(
        &self,
        username: &str,
        password: &str,
        role: models::user::Role,
    ) -> Result<i64, AppError>;

    async fn login(&self, username: &str, password: &str) -> Result<i64, AppError>;

    async fn get_user(&self, user_id: i64) -> Result<models::user::User, AppError>;
}
