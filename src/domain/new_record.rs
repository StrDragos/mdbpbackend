use crate::domain::enums::RecordType;
use crate::grpc::records::CreateRecordInput;
use crate::grpc::records::RecordType as RecordTypeGrpc;
use crate::grpc::transformations::timestamp_to_chrono;
use chrono::DateTime;
use uuid::Uuid;
use crate::error::AppError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewRecord {
    pub title: String,
    pub subtitle: String,
    pub record_type: RecordType,
    pub create_date: DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub facility_name: String,
    pub user_id: String,
    pub bytes: Vec<u8>,
    pub stored_resource: String,
}

impl NewRecord {
    pub fn try_from(input: CreateRecordInput, stored_file_name: &str) -> Result<Self, AppError> {
        let record_type = RecordTypeGrpc::try_from(input.r#type)
            .map_err(|e| AppError::Validation(format!("Wrong input record type {}", e)))
            .and_then(|t| t.try_into())?;

        let create_date = input
            .date
            .ok_or(AppError::Validation("Date is required for new records".to_string()))
            .and_then(|t| timestamp_to_chrono(t))?;

        Ok(NewRecord {
            title: input.title.clone(),
            subtitle: input.subtitle.clone(),
            record_type,
            create_date,
            tags: input.tags.clone(),
            facility_name: input.facility_name.unwrap(),
            user_id: input.user_id.clone(),
            bytes: input.file_data,
            stored_resource: stored_file_name.to_string(),
        })
    }
}
