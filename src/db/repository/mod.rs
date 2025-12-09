use crate::{models, utils::error::AppError};

pub mod users;

pub trait UserRepo {
    async fn create_user<'a>(
        &'a self,
        username: &'a str,
        password: &'a str,
        role: models::user::Role,
    ) -> Result<i64, AppError>;

    async fn login(&self, username: &str, password: &str) -> Result<i64, AppError>;

    async fn get_user(&self, user_id: i64) -> Result<models::user::User, AppError>;
}
