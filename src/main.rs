mod config;
mod domain;
mod error;
mod grpc;
mod handlers;
mod infrastructure;

use crate::config::application::{AppConfig, AppEnv};
use crate::grpc::record_service::RecordServiceImpl;
use crate::handlers::records::RecordHandlerImpl;
use crate::infrastructure::db;
use crate::infrastructure::db::connection;
use crate::infrastructure::files_storage::FilesStorageLive;
use crate::infrastructure::repositories::records_repository::RecordsRepositoryImpl;
use aws_config::BehaviorVersion;
use aws_config::profile::load;
use aws_sdk_s3::Client;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::{debug, info};
use tracing_subscriber::fmt::format;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::build()?;

    //Repositories
    let conn_pool = connection(&config.db_config).await;
    let db_connection = Arc::new(conn_pool);
    let records_repository = Arc::new(RecordsRepositoryImpl::new(db_connection.clone()));

    let storage = FilesStorageLive::from_config(&config.storage_config).await;
    let storage_arc = Arc::new(storage);

    //Records handler
    let record_service_impl = Arc::new(RecordHandlerImpl::new(
        storage_arc.clone(),
        records_repository.clone(),
    ));

    //enable logging
    match &config.environment {
        AppEnv::Local => tracing_subscriber::fmt::init(),
        AppEnv::Production => (), //For now
    }
    info!("Starting server");

    debug!("Config: {:#?}", config);
    let address = format!("127.0.0.1:{}", config.server_port).parse()?;
    Server::builder()
        .add_service(RecordServiceImpl::server(record_service_impl.clone()))
        .serve(address)
        .await?;

    Ok(())
}
