use crate::grpc::records::{
    CreateRecordRequest, CreateRecordResponse, DeleteRecordRequest, DeleteRecordResponse,
    GetRecordRequest, GetRecordResponse, ListRecordsRequest, ListRecordsResponse,
    UpdateRecordRequest, UpdateRecordResponse,
};
use crate::grpc::{records, response};
use crate::handlers;
use std::sync::Arc;
use tonic::{Request, Response, Status, async_trait};
use tonic::codegen::InterceptedService;
use tracing::log::info;
use crate::grpc::authorization::AuthInterceptor;
use crate::grpc::records::records_service_server::{RecordsService, RecordsServiceServer};

#[derive(Clone)]
pub struct RecordServiceImpl {
    handler: Arc<dyn handlers::records::RecordHandlers>,
}

impl RecordServiceImpl {
    pub fn new(handler: Arc<handlers::records::RecordHandlerImpl>) -> Self {
        Self { handler }
    }
    pub fn server(
        records_service: Arc<handlers::records::RecordHandlerImpl>
    ) -> InterceptedService<RecordsServiceServer<RecordServiceImpl>, AuthInterceptor>  {
        let service =  RecordServiceImpl::new(records_service.clone());
        RecordsServiceServer::with_interceptor(service, AuthInterceptor)
    }
}

#[async_trait]
impl RecordsService for RecordServiceImpl {
    async fn create_record(
        &self,
        request: Request<CreateRecordRequest>,
    ) -> Result<Response<CreateRecordResponse>, Status> {
        info!("Grpc request received");
        let create_record_req = request.into_inner();
        let handler_result = self.handler.save(create_record_req).await;
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
