mod cli;
pub mod commands;
mod utils;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Diff { original, modified } => commands::diff::run(&original, &modified),
        Commands::Info => {
            commands::info::run();
            Ok(())
        }
    }
}
