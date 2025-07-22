
use clap::{Parser, Subcommand};

/// Docdiff CLI arguments and subcommands
#[derive(Parser)]
#[command(
    name = "docdiff",
    about = "Effortlessly compare two text files using document distance algorithms.",
    long_about = "Docdiff is a fast, intuitive CLI for comparing two .txt files. Get instant feedback on how similar or different your documents are, with friendly messages and clear output."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compare two .txt files and see how similar or different they are.
    Diff {
        /// Path to the original .txt file. Example: file1.txt
        original: String,
        /// Path to the modified .txt file. Example: file2.txt
        modified: String,
    },
    /// Show information about the program.
    Info,
}

