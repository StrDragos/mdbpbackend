use crate::domain::enums::RecordType;
use crate::domain::new_record::NewRecord;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct RecordRow {
    pub id: Uuid,
    pub user_id: String,
    pub title: String,
    pub file_name: String,
    pub record_type: RecordType,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub facility_name: String,
    pub subtitle: String,
}

impl From<NewRecord> for RecordRow {
    fn from(value: NewRecord) -> Self {
        RecordRow {
            id: Uuid::now_v7(),
            user_id: value.user_id.to_string(),
            title: value.title,
            subtitle: value.subtitle,
            file_name: value.stored_resource,
            record_type: value.record_type,
            created_at: value.create_date,
            updated_at: None,
            tags: value.tags,
            facility_name: value.facility_name,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::enums::RecordType;
    use crate::domain::new_record::NewRecord;
    use crate::infrastructure::models::record_row::RecordRow;
    use chrono::DateTime;
    use uuid::Uuid;

    #[test]
    fn test_record_row_from_new_record() {
        let new_record = NewRecord {
            title: "Test".to_string(),
            subtitle: "Test".to_string(),
            record_type: RecordType::Other,
            create_date: DateTime::UNIX_EPOCH,
            tags: Vec::new(),
            facility_name: "Test facility".to_string(),
            user_id: "test_user_id".to_string(),
            bytes: Vec::new(),
            stored_resource: "Test".to_string(),
        };

        let record_row = RecordRow::from(new_record);
        assert_eq!(record_row.title, "Test");
        assert_eq!(record_row.subtitle, "Test");
        assert_eq!(record_row.record_type, RecordType::Other);
        assert_eq!(record_row.created_at, DateTime::UNIX_EPOCH);
        assert_eq!(record_row.facility_name, "Test facility");
        assert_eq!(record_row.file_name, "Test");
    }
}
