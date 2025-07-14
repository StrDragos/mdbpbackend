use crate::domain::user::User;
use crate::error::AppError;
use crate::grpc::transformations::timestamp_to_chrono;
use crate::grpc::users::RegisterRequest;
use crate::infrastructure::repositories::users_repository::UserRepository;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use std::sync::Arc;
use tonic::async_trait;

#[async_trait]
pub trait UserHandler: Send + Sync {
    async fn register(&self, request: RegisterRequest) -> Result<User, AppError>;
}

pub struct UserHandlerImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserHandlerImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserHandler for UserHandlerImpl {
    async fn register(&self, request: RegisterRequest) -> Result<User, AppError> {
        let date_of_brith = request
            .date_of_birth
            .ok_or(AppError::Validation(
                "Date of birth is mandatory".to_string(),
            ))
            .and_then(|timestamp| timestamp_to_chrono(timestamp))?;

        self.user_repository
            .register_new_user(
                &request.user_id,
                &request.email,
                &request.name,
                date_of_brith,
            )
            .await
    }
}
