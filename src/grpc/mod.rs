pub mod record_service;

pub mod records {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/generated/medpass.records.v1.rs"
    ));
}
pub mod transformations {
    use crate::error::AppError;
    use chrono::{DateTime, TimeZone, Utc};
    use prost_types::Timestamp;

    pub fn timestamp_to_chrono(timestamp: Timestamp) -> Result<DateTime<Utc>, AppError> {
        Utc.timestamp_opt(timestamp.seconds.clone(), timestamp.nanos.clone() as u32)
            .single()
            .ok_or(AppError::Internal(format!("Failed to deserialize timestamp {}", timestamp)))
    }
}
