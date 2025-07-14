use crate::error::AppError;
use crate::grpc::records::RecordType as GrpcRecordType;
use sqlx::Type;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Type, EnumString, Display)]
#[sqlx(type_name = "TEXT")]
#[strum(serialize_all = "snake_case")]
pub enum RecordType {
    LabResults,
    Imaging,
    Visit, //usually for medical letters
    Other,
}

impl TryFrom<GrpcRecordType> for RecordType {
    type Error = AppError;

    fn try_from(value: GrpcRecordType) -> Result<Self, Self::Error> {
        match value {
            GrpcRecordType::LabResult => Ok(RecordType::LabResults),
            GrpcRecordType::Visit => Ok(RecordType::Visit),
            GrpcRecordType::Imaging => Ok(RecordType::Imaging),
            GrpcRecordType::Other => Ok(RecordType::Other),
            GrpcRecordType::Unspecified => {
                Err(AppError::Validation("RecordType Unspecified".to_string()))
            }
            _ => Err(AppError::Validation(
                "RecordType not yet supported".to_string(),
            )),
        }
    }
}
