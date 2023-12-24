use alqs_shared::tables::tables_service_server::TablesService;
use alqs_shared::tables::{CreateTableReply, CreateTableRequest};
use alqs_shared::tonic::{Request, Response, Status};

#[derive(Default)]
pub struct AlqsTable {}

#[alqs_shared::tonic::async_trait]
impl TablesService for AlqsTable {
    async fn create_table(
        &self,
        request: Request<CreateTableRequest>,
    ) -> Result<Response<CreateTableReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = alqs_shared::tables::CreateTableReply {
            ok: true,
            errors: vec![],
        };
        Ok(Response::new(reply))
    }
}
