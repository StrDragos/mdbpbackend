use crate::grpc::{records, response};
use crate::grpc::records::{
    CreateRecordRequest, CreateRecordResponse, DeleteRecordRequest, DeleteRecordResponse,
    GetRecordRequest, GetRecordResponse, ListRecordsRequest, ListRecordsResponse,
    UpdateRecordRequest, UpdateRecordResponse,
};
use crate::handlers;
use crate::grpc::convert::*;
use std::sync::Arc;
use tonic::{Request, Response, Status, async_trait};
use tracing::error;
use tracing::log::{Record, info};
use crate::domain::user_record::UserRecord;
use crate::error::AppError;
// In src/lib.rs or src/main.rs

// medpass.records.v1
pub struct RecordServiceImpl {
    records_service: Arc<handlers::records::RecordHandlerImpl>,
}

impl RecordServiceImpl {
    pub fn server(
        records_service: Arc<handlers::records::RecordHandlerImpl>,
    ) -> records::records_service_server::RecordsServiceServer<Self> {
        records::records_service_server::RecordsServiceServer::new(RecordServiceImpl {
            records_service,
        })
    }
}

#[async_trait]
impl records::records_service_server::RecordsService for RecordServiceImpl {
    async fn create_record(
        &self,
        request: Request<CreateRecordRequest>,
    ) -> Result<Response<CreateRecordResponse>, Status> {
        info!("Grpc request received");
        let create_record_req = request.into_inner();
        let handler_result = self.records_service.save(create_record_req).await;
        
        response::to_response(handler_result)
    }


    async fn get_record(
        &self,
        request: Request<GetRecordRequest>,
    ) -> Result<Response<GetRecordResponse>, Status> {
        todo!()
    }

    async fn list_records(
        &self,
        request: Request<ListRecordsRequest>,
    ) -> Result<Response<ListRecordsResponse>, Status> {
        todo!()
    }

    async fn update_record(
        &self,
        request: Request<UpdateRecordRequest>,
    ) -> Result<Response<UpdateRecordResponse>, Status> {
        todo!()
    }

    async fn delete_record(
        &self,
        request: Request<DeleteRecordRequest>,
    ) -> Result<Response<DeleteRecordResponse>, Status> {
        todo!()
    }
}
