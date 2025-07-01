use crate::domain::new_record::NewRecord;
use crate::infrastructure::models::record_row::RecordRow;
use sqlx::PgPool;
use std::sync::Arc;
use tonic::async_trait;
use tracing::error;
use tracing::field::debug;
use tracing::log::debug;
use crate::error::AppError;

#[async_trait]
pub trait RecordsRepository: Send + Sync {
    async fn save(&self, record: NewRecord) -> Result<(), AppError>;
    async fn get(&self) -> Result<String, AppError>;
    async fn delete(&self) -> Result<String, AppError>;
    async fn update(&self, record: String) -> Result<(),  AppError>;
}

pub struct RecordsRepositoryImpl {
    pool: Arc<PgPool>,
}
#[async_trait]
impl RecordsRepository for RecordsRepositoryImpl {
    async fn save(&self, record: NewRecord) -> Result<(), AppError> {

        let row: RecordRow = record.into();
        debug!("Saving {:#?}", row);

        let query = r#"INSERT INTO records (
            id,
            user_id,
            file_name,
            record_type,
            created_at,
            facility_name,
            title,
            subtitle,
            tags
        ) VALUES (
            $1,$2,$3,$4,$5,$6,$7,$8,$9
        )
        RETURNING  id,
            user_id,
            file_name,
            record_type,
            created_at,
            facility_name,
            title,
            subtitle,
            tags"#;

        let query_builder = sqlx::query_as::<_, RecordRow>(query)
            .bind(&row.id)
            .bind(&row.user_id)
            .bind(&row.file_name)
            .bind(&row.record_type)
            .bind(&row.created_at)
            .bind(&row.facility_name)
            .bind(&row.title)
            .bind(&row.subtitle)
            .bind(&row.tags);


        query_builder
            .fetch_one(&*self.pool)
            .await
            .map_err(|err|{
                error!(error= ?err, file_location = %row.file_name, user_id= %row.user_id, "Failed to save");
                AppError::DatabaseError("could not save new record".to_string())
            })
            //TODO return saved record
            .map(|_| ())
    }

    async fn get(&self) -> Result<String, AppError> {
        todo!()
    }

    async fn delete(&self) -> Result<String, AppError> {
        todo!()
    }

    async fn update(&self, record: String) -> Result<(), AppError> {
        todo!()
    }
}

impl RecordsRepositoryImpl {
    pub fn new(connection: Arc<PgPool>) -> Self {
        Self { pool: connection }
    }
}
