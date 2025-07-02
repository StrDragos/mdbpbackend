use crate::domain::enums::RecordType;
use crate::domain::user_record::UserRecord;
use crate::grpc::records::{CreateRecordResponse, Record as GrpcRecord};
use crate::grpc::records::RecordType as GrpcRecordType;
use crate::grpc::transformations::chrono_to_timestamp;

impl From<UserRecord> for CreateRecordResponse {
    fn from(value: UserRecord) -> Self {
        let r = GrpcRecord {
            id: value.id.to_string(),
            r#type: value.record_type.into(),
            title: value.title,
            subtitle: value.subtitle,
            date: Some(chrono_to_timestamp(value.create_date)),
            tags: value.tags,
            document_url: value.stored_resource,
            facility_name: value.facility_name,
            notes: None
        };
        CreateRecordResponse{
            record: Some(r)
        }
    }
}

impl From<RecordType> for i32 {
    fn from(value: RecordType) -> Self {
        match value {
            RecordType::LabResults => GrpcRecordType::LabResult as i32,
            RecordType::Imaging => GrpcRecordType::Imaging as i32,
            RecordType::Visit => GrpcRecordType::Visit as i32,
            RecordType::Other => GrpcRecordType::Other as i32
        }
    }
}