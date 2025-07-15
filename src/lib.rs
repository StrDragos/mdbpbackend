use std::sync::Arc;
use aws_sdk_s3::types::BucketCannedAcl::AuthenticatedRead;
use sqlx::{Pool, Postgres};
use tonic::transport::Server;
use tracing::{debug, info};
use crate::config::application::{AppConfig, AppEnv};
use crate::grpc::authorization::AuthInterceptor;
use crate::grpc::record_service::RecordServiceImpl;
use crate::grpc::user_service::UserServiceImpl;
use crate::handlers::records::RecordHandlerImpl;
use crate::handlers::users::UserHandlerImpl;
use crate::infrastructure::files_storage::FilesStorageLive;
use crate::infrastructure::repositories::records_repository::RecordsRepositoryImpl;
use crate::infrastructure::repositories::users_repository::UserRepositoryImpl;

pub mod config;
pub mod domain;
pub mod error;
pub mod grpc;
pub mod handlers;
pub mod infrastructure;
pub async fn run(config: AppConfig, connection_pool: Pool<Postgres>) -> Result<(),  Box<dyn std::error::Error>> {

    let db_connection = Arc::new(connection_pool);
    let records_repository = Arc::new(RecordsRepositoryImpl::new(db_connection.clone()));
    let user_repository = Arc::new(UserRepositoryImpl::new(db_connection.clone()));

    let storage = FilesStorageLive::from_config(&config.storage_config).await;
    let storage_arc = Arc::new(storage);

    //Records handler
    let record_service_impl = Arc::new(RecordHandlerImpl::new(
        storage_arc.clone(),
        records_repository.clone(),
    ));

    //User service
    let user_service_impl = Arc::new(UserHandlerImpl::new(user_repository.clone()));

    //enable logging
    match &config.environment {
        AppEnv::Local => tracing_subscriber::fmt::init(),
        AppEnv::Production => (), //For now
    }

    let port = config.server_port.unwrap_or(0);
    debug!("Config: {:#?}", config);
    let address = format!("127.0.0.1:{}", port).parse()?;
    info!("Starting server from {}", address);

    let _ = Server::builder()
        .add_service(RecordServiceImpl::server(record_service_impl.clone()))
        .add_service(UserServiceImpl::server(user_service_impl.clone()))
        .serve(address).await.unwrap();

    Ok(())
}
