use alqs_shared::tokio;
use clap::Parser;

mod commands;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let output = cli.command.run().await?;

    println!("{}", output);

    Ok(())
}
