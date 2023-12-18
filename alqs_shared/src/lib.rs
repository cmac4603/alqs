pub use tokio;
pub use tonic;
pub use tonic_health;

pub mod status {
    tonic::include_proto!("status");
}

pub mod tables {
    tonic::include_proto!("tables");
}
