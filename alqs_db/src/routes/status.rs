use alqs_shared::status::status_service_server::StatusService;
use alqs_shared::status::{StatusReply, StatusRequest};
use alqs_shared::tonic::{Request, Response, Status};

const ALQS_DB_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct AlqsStatus {}

#[alqs_shared::tonic::async_trait]
impl StatusService for AlqsStatus {
    async fn check_status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = alqs_shared::status::StatusReply {
            status: format!("ALQS DB v{}", ALQS_DB_VERSION),
        };
        Ok(Response::new(reply))
    }
}
