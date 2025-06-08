use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "sui-move-kit", version = "0.1.0", about = "CLI toolkit for Sui Move development")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(help = "Initialize a new Move project")]
        name: String,
    },
    Build,
    Test,
    Publish {
        #[arg(help = "Sui address or endpoint")]
        endpoint: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            println!("Initializing new Move project: {}", name);
            // logic here
        }
        Commands::Build => {
            println!("Building Move project...");
            // logic here
        }
        Commands::Test => {
            println!("Running tests...");
            // logic here
        }
        Commands::Publish { endpoint } => {
            println!("Publishing to {:?}", endpoint);
            // logic here
        }
    }

    Ok(())
}