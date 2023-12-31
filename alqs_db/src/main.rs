use alqs_shared::status::status_service_server::StatusServiceServer;
use alqs_shared::tokio;
use alqs_shared::tonic::transport::Server;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) =
        alqs_shared::tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<StatusServiceServer<routes::status::AlqsStatus>>()
        .await;

    let addr = "[::1]:50051".parse().unwrap();

    println!("ALQS DB listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(StatusServiceServer::new(
            routes::status::AlqsStatus::default(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
