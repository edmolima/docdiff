pub mod commands;
mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Diff { original, modified } => {
            commands::diff::run(&original, &modified)
        }
        Commands::Info => {
            commands::info::run();
            Ok(())
        }
    }
}
