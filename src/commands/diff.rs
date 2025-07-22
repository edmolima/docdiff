use anyhow::Result;
use crate::utils;
use crate::algorithm::document_distance;

pub fn run(original: &str, modified: &str) -> Result<()> {
    println!("Comparing '{original}' with '{modified}'...");
    let text_a = utils::read_file(original)?;
    let text_b = utils::read_file(modified)?;

    let pb = indicatif::ProgressBar::new(100);
    pb.inc(100);
    pb.finish_with_message("Comparison finished!");

    let distance = document_distance::document_distance(&text_a, &text_b);
    println!("Document distance: {distance:.3}");
    Ok(())
}
