use crate::domain::enums::RecordType;
use uuid::Uuid;

pub struct UserRecord {
    pub id: Uuid,
    pub title: String,
    pub subtitle: String,
    pub record_type: RecordType,
    pub create_date: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub facility_name: String,
    pub user_id: String,
    pub stored_resource: String,
}
