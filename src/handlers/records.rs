use crate::domain::new_record::NewRecord;
use crate::grpc::records::CreateRecordRequest;
use crate::infrastructure::files_storage::FilesStorage;
use crate::infrastructure::repositories::records_repository::RecordsRepository;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tonic::async_trait;
use tracing::info;
use tracing::log::error;
use crate::domain::user_record::UserRecord;
use crate::error::AppError;

#[async_trait]
trait RecordHandlers: Send + Sync {
    async fn save(&self, create: CreateRecordRequest);
}

pub struct RecordHandlerImpl {
    storage: Arc<dyn FilesStorage>,
    records_repo: Arc<dyn RecordsRepository>,
}

impl RecordHandlerImpl {
    pub fn new(storage: Arc<dyn FilesStorage>, records_repo: Arc<dyn RecordsRepository>) -> Self {
        Self {
            storage,
            records_repo,
        }
    }

    pub async fn save(&self, create: CreateRecordRequest) -> Result<UserRecord, AppError> {
        info!("Received request");
        match create.record {
            Some(input_record) => {
                info!("Record found for user {}", input_record.user_id);
                let file_name =
                    Self::compute_file_name(&input_record.file_data[..], &input_record.user_id);
                info!("File name {}", &file_name);
                let file_metadata: NewRecord =
                    NewRecord::try_from(input_record.clone(), &file_name)?;
                let data = input_record.file_data;

                //It might be worth looking into streaming the file to the storage
                //For simplicity we will just store the file in memory
                info!("Send file {} to storage", &file_name);
                let _ = self
                    .storage
                    .save_file(data, &file_name)
                    .await?;

                match self.records_repo.save(file_metadata).await {
                    Ok(result) => {
                        info!("Record saved");
                        Ok(result)
                    },
                    Err(e) => {
                        error!("Failed to save record {}, we will rollback stored record", e);
                        self.storage.delete_file(&file_name).await?;
                        Err(AppError::Internal("Failed to save metadata".to_string()))
                    },
                }
            }
            None => Err(AppError::Validation("record is a required value".to_string())),
        }
    }

    fn compute_file_name(record_data: &[u8], user_id: &str) -> String {
        let hasher = Sha256::digest(record_data);
        let name = hex::encode(hasher);
        //TODO handle file type (.pdf, .jpg ...etc)
        format!("{}/{}.pdf", user_id, name)
    }
}
