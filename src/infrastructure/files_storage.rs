use crate::config::application::StorageConfig;
use crate::error::AppError;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use bytes;
use tonic::async_trait;
use tracing::{debug, error, info};

#[async_trait]
pub trait FilesStorage: Send + Sync {
    async fn save_file(&self, file: Vec<u8>, name: &str) -> Result<(), AppError>;
    async fn get_file(&self, file_name: &str) -> Result<Vec<u8>, AppError>;
    async fn delete_file(&self, file_name: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct FilesStorageLive {
    client: Client,
    bucket_name: String,
}

impl FilesStorageLive {
    pub fn new(client: Client, bucket_name: String) -> Self {
        Self {
            client,
            bucket_name,
        }
    }

    pub async fn from_config(storage_config: &StorageConfig) -> Self {
        let s3_config = aws_config::defaults(BehaviorVersion::v2025_01_17())
            .endpoint_url(&storage_config.bucket_url)
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                &storage_config.access_key_id,
                &storage_config.secret_access_key,
                None,
                None,
                "custom",
            ))
            .region(aws_sdk_s3::config::Region::new(
                storage_config.region.clone(),
            ))
            .load()
            .await;

        let s3_client = Client::new(&s3_config);
        FilesStorageLive::new(s3_client, storage_config.bucket_name.clone())
    }
}

#[async_trait]
impl FilesStorage for FilesStorageLive {
    async fn save_file(&self, file: Vec<u8>, name: &str) -> Result<(), AppError> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket_name)
            .key(name)
            .send()
            .await
        {
            Ok(_) => {
                debug!("File {} already exists", &name);
                Err(AppError::StorageError(
                    "Medical document could not be stored due to duplication".to_string(),
                ))
            }
            Err(e) => {
                let service_error = e.as_service_error();
                if let Some(HeadObjectError::NotFound(_)) = service_error {
                    info!("File {} not found, we will save it", &name);
                    self.client
                        .put_object()
                        .bucket(&self.bucket_name)
                        .key(name)
                        .body(ByteStream::from(bytes::Bytes::from(file)))
                        .send()
                        .await
                        .map(|_| info!("File {} not saved", &name))
                        .map_err(|e| {
                            error!("Failed to save object: {}", e);
                            AppError::StorageError("Failed to store file".to_string())
                        })
                } else {
                    error!("Failed to get information about existing file: {:#?}", e);
                    Err(AppError::StorageError("Failed to store file".to_string()))
                }
            }
        }
    }

    async fn get_file(&self, file_name: &str) -> Result<Vec<u8>, AppError> {
        todo!()
    }

    async fn delete_file(&self, file_name: &str) -> Result<(), AppError> {
        self.client
            .delete_object()
            .key(file_name)
            .bucket(&self.bucket_name)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to delete object: {}", e);
                AppError::StorageError("Failed to delete file".to_string())
            })
            .map(|_| info!("File {} deleted", &file_name))
    }
}
