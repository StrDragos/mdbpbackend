use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::domain::user::User;

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub date_of_birth: DateTime<Utc>
}


impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            user_id: value.id,
            full_name: value.full_name,
            email: value.email,
            date_of_birth: value.date_of_birth
        }
    }
}