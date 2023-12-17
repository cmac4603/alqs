use alqs_shared::status::status_service_client::StatusServiceClient;
use alqs_shared::status::StatusRequest;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) async fn status() -> Result<String, Box<dyn std::error::Error>> {
    let mut client = StatusServiceClient::connect("http://[::1]:50051").await?;

    let request = alqs_shared::tonic::Request::new(StatusRequest {});

    let response = client.check_status(request).await?;

    Ok(format!(
        "ALQS CLI v{}\n{}",
        VERSION,
        response.into_inner().status
    ))
}
