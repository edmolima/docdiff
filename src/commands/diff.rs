use crate::utils;
use anyhow::Result;
use colored::*;
use docdiff::algorithm::document_distance;
use std::time::Duration;

pub fn run(original: &str, modified: &str) -> Result<()> {
    println!(
        "\n{} Comparing '{}' with '{}'...",
        "🔍".blue().bold(),
        original.green(),
        modified.green()
    );
    let text_a = utils::read_file(original)?;
    let text_b = utils::read_file(modified)?;

    let pb = indicatif::ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Analyzing files...");

    let distance = document_distance::document_distance(&text_a, &text_b);

    pb.finish_with_message("Done!");

    println!();

    let similarity = ((1.0 - distance) * 100.0).round();

    let verdict = match distance {
        _ if distance < 0.05 => "✅ The files are almost identical.".green(),
        _ if distance < 0.25 => "👍 The files are very similar.".bright_green(),
        _ if distance < 0.50 => "🟡 The files share some similarities.".yellow(),
        _ if distance < 0.75 => "⚠️ The files are quite different.".magenta(),
        _ => "❌ The files are very different.".red(),
    };

    println!("{}", "Result:".bold().underline());
    println!(
        "Document distance: {} (Similarity: {}%)",
        format!("{distance:.3}").cyan(),
        format!("{similarity:.0}").cyan()
    );
    println!("{}\n", verdict.bold());
    Ok(())
}
