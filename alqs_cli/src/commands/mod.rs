use alqs_shared::tables::Column;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod status;
mod tables;

#[derive(Debug, Clone)]
pub(crate) struct AlqsTableColumn(Column);

fn parse_column(input: &str) -> Result<AlqsTableColumn, std::io::Error> {
    Ok(AlqsTableColumn(Column {
        name: "test".to_string(),
        nullable: false,
        primary_key: false,
        r#type: 1,
    }))
}

/// ALQS CLI commands
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "create-table", about = "Create ALQS tables")]
    CreateTable {
        name: String,
        #[arg(long, value_parser = clap::builder::ValueParser::new(parse_column))]
        column: Vec<AlqsTableColumn>,
    },
    #[command(about = "Check the status of the ALQS DB server")]
    Status,
}

impl Command {
    pub(crate) async fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Command::CreateTable { name, column } => {
                // let table_command = TableCommand::parse();
                // table_command.run().await
                Ok("Table command".to_string())
            }
            Command::Status => status::status().await,
        }
    }
}
