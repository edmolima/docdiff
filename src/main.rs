
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "docdiff")]
#[command(about = "CLI tool for comparing documents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compare two text files
    Diff {
        /// Path to the original file
        original: String,
        /// Path to the modified file
        modified: String,
    },
    /// Show information about the program
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Diff { original, modified } => {
            println!("Comparing '{original}' with '{modified}'...");
            let pb = indicatif::ProgressBar::new(100);
            pb.inc(100);
            pb.finish_with_message("Comparison finished!");
        }
        Commands::Info => {
            println!("docdiff v0.1.0 - CLI for document comparison");
        }
    }
    Ok(())
}
