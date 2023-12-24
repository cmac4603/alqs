use alqs_shared::tables::Column;
use clap::Subcommand;

mod status;
mod tables;

#[derive(Debug, Clone)]
pub(crate) struct AlqsTableColumn(Column);

fn parse_column(input: &str) -> Result<AlqsTableColumn, std::io::Error> {
    let col_data = input.split(',').collect::<Vec<&str>>();
    let column = Column {
        name: col_data[0].to_string(),
        nullable: col_data[1].parse::<bool>().unwrap(),
        primary_key: col_data[2].parse::<bool>().unwrap(),
        r#type: col_data[3].parse::<i32>().unwrap(),
    };
    return Ok(AlqsTableColumn(column));
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
                dbg!(column);
                Ok("Table command".to_string())
            }
            Command::Status => status::status().await,
        }
    }
}
