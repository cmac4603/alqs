use alqs_shared::hello_world::greeter_client::GreeterClient;
use alqs_shared::hello_world::HelloRequest;
use alqs_shared::tokio;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = alqs_shared::tonic::Request::new(HelloRequest { name: args.name });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
