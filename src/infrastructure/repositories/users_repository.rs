use std::sync::Arc;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tonic::async_trait;
use crate::domain::user::User;
use crate::error::AppError;
use crate::infrastructure::models::user_row::UserRow;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn  register_new_user(&self, user_id: &str, email: &str, full_name: &str, date_of_birth: DateTime<Utc>) ->  Result<User, AppError>;

    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, AppError>;
}

pub struct UserRepositoryImpl{
    pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn register_new_user(&self, user_id: &str, email: &str, full_name: &str, date_of_birth: DateTime<Utc>) -> Result<User, AppError> {
        let query = "INSERT INTO users (id, email, full_name, date_of_birth, created_at) VALUES ($1, $2, $3) RETURNING *";
        sqlx::query_as::<_, UserRow>(query)
            .bind(user_id)
            .bind(email)
            .bind(full_name)
            .bind(date_of_birth)
            .bind(Utc::now())
            .fetch_one(&*self.pool)
            .await
            .map_err(|err|{
                tracing::error!(error = ?err, "Failed to create user {user_id:?}");
                AppError::DatabaseError(format!("Failed to save user {user_id:?}"))
            })
            .map(|row| row.into())
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&*self.pool).await
            .map(|maybe_row| maybe_row.map(|row| row.into()))
            .map_err(|err|{
                tracing::error!(error = ?err, "Failed to fetch user {user_id:?}");
                AppError::DatabaseError(format!("Failed to fetch user {user_id:?}"))
            })

    }
}