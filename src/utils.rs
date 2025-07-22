use anyhow::{Context, Result};
use std::fs;

/// Reads the entire contents of a file as a String.
pub fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {path}"))
}
