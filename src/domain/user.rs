use chrono::{DateTime, Utc};

pub struct User {
    pub full_name: String,
    pub email: String,
    pub date_of_birth: DateTime<Utc>,
    pub user_id: String,
} 