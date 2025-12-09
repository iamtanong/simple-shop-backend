use sqlx::{query, query_as, PgPool};

use crate::db::repository::UserRepo;
use crate::models;
use crate::utils::error::AppError;

pub struct UserRepository {
    db_pool: PgPool,
}

impl UserRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

impl UserRepo for UserRepository {
    async fn create_user(
        &self,
        username: &str,
        password: &str,
        role: crate::models::user::Role,
    ) -> Result<i64, AppError> {
        let id = match query!(
            r#"
            INSERT INTO users (username, password, role)
            VALUES ($1, $2, $3)
            RETURNING id;
        "#,
            username,
            password,
            role.to_string(),
        )
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(result) => result.id,
            Err(e) => {
                log::error!("{}", e);
                return Err(AppError::InternalServerError);
            }
        };

        Ok(id)
    }

    async fn login(&self, username: &str, password: &str) -> Result<i64, AppError> {
        todo!()
    }

    async fn get_user(&self, user_id: i64) -> Result<models::user::User, AppError> {
        let result = query_as!(
            models::user::User,
            r#"
            SELECT username, role 
            FROM users 
            WHERE id=$1 AND active=true;
        "#,
            user_id,
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|_err| AppError::InternalServerError);

        result
    }
}
