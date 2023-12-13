use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get an executable or script
    Get,

    /// Upload an executable or script to the repository
    Upload,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get => {
            todo!("Get the executable")
        },
        Commands::Upload => {
            todo!("Upload the executable")
        }
    }
}