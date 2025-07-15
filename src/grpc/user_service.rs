use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use tonic::codegen::InterceptedService;
use crate::grpc::authorization::AuthInterceptor;
use crate::grpc::response;
use crate::grpc::users::{RegisterRequest, RegisterResponse};
use crate::grpc::users::user_service_server::{UserService, UserServiceServer};
use crate::handlers::users::UserHandler;

pub struct UserServiceImpl{
    handler: Arc<dyn UserHandler>
}

impl UserServiceImpl{
    pub fn server(handler: Arc<dyn UserHandler>) -> InterceptedService<UserServiceServer<UserServiceImpl>, AuthInterceptor>{
        UserServiceServer::with_interceptor(UserServiceImpl{handler}, AuthInterceptor::default())
    }
}

#[async_trait]
impl UserService for UserServiceImpl{
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let register_req = request.into_inner();
        let handled_value = self.handler.register(register_req).await;
        response::to_response(handled_value)
    }
}


