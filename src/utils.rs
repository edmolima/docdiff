use anyhow::{Context, Result};
use std::fs;

pub fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {path}"))
}
