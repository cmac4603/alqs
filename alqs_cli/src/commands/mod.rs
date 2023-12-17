use clap::Subcommand;

mod status;

/// Simple program to greet a person
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Status,
}

impl Command {
    pub(crate) async fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Command::Status => status::status().await,
        }
    }
}
