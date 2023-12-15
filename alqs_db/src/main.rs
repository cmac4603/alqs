use alqs_shared::hello_world::greeter_server::{Greeter, GreeterServer};
use alqs_shared::hello_world::{HelloReply, HelloRequest};
use alqs_shared::tokio;
use alqs_shared::tonic::{transport::Server, Request, Response, Status};
use alqs_shared::tonic_health::server::HealthReporter;

#[derive(Default)]
pub struct MyGreeter {}

#[alqs_shared::tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = alqs_shared::hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) =
        alqs_shared::tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<GreeterServer<MyGreeter>>()
        .await;

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("ALQS DB listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
