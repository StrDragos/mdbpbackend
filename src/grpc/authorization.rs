use tonic::{Request, Status};
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;


#[derive(Default, Clone)]
pub struct AuthInterceptor;
impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

        match request.metadata().get("authorization") {
            Some(t) if token == t => Ok(request),
            _ => Err(Status::unauthenticated("No valid auth token")),
        }
    }
}
