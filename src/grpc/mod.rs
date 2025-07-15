pub mod convert;
pub mod record_service;
pub mod user_service;
pub mod authorization;

pub mod records {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/generated/medpass.records.v1.rs"
    ));
}
pub mod users {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/generated/medpass.users.v1.rs"
    ));
}
pub mod transformations {
    use crate::error::AppError;
    use chrono::{DateTime, TimeZone, Utc};
    use prost_types::Timestamp;

    pub fn timestamp_to_chrono(timestamp: Timestamp) -> Result<DateTime<Utc>, AppError> {
        Utc.timestamp_opt(timestamp.seconds.clone(), timestamp.nanos.clone() as u32)
            .single()
            .ok_or(AppError::Internal(format!(
                "Failed to deserialize timestamp {}",
                timestamp
            )))
    }

    pub fn chrono_to_timestamp(timestamp: DateTime<Utc>) -> Timestamp {
        Timestamp {
            seconds: timestamp.timestamp(),
            nanos: timestamp.timestamp_subsec_nanos() as i32,
        }
    }
}

pub mod response {
    use crate::error::AppError;
    use tonic::{Response, Status};
    use tracing::error;

    pub fn to_response<T, G: From<T>>(value: Result<T, AppError>) -> Result<Response<G>, Status> {
        value.map(|t| Response::new(G::from(t))).map_err(|e| {
            error!(error=%e, "Request failed");
            match e {
                AppError::Internal(err) => Status::internal(err),
                AppError::StorageError(msg) => Status::internal(msg),
                AppError::DatabaseError(msg) => Status::internal(msg),
                AppError::Validation(msg) => Status::invalid_argument(msg),
                AppError::UserAlreadyExists(msg) => Status::already_exists(msg),
            }
        })
    }
}
