use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::grpc::response;
use crate::grpc::users::{RegisterRequest, RegisterResponse};
use crate::grpc::users::user_service_server::UserService;
use crate::handlers::users::UserHandler;

pub struct UserServiceImpl{
    handler: Arc<dyn UserHandler>
}

impl UserServiceImpl{
    pub fn new(handler: Arc<dyn UserHandler>) -> Self{
        Self{handler}
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


