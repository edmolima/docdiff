use anyhow::Result;
use crate::utils;
use docdiff::algorithm::document_distance;
use colored::*;

pub fn run(original: &str, modified: &str) -> Result<()> {
    println!("\n{} Comparing '{}' with '{}'...", "🔍".blue().bold(), original.green(), modified.green());
    let text_a = utils::read_file(original)?;
    let text_b = utils::read_file(modified)?;

    let pb = indicatif::ProgressBar::new(100);
    pb.set_message("Analyzing files...");
    pb.inc(100);
    pb.finish_with_message("Done!");

    println!("");

    let distance = document_distance::document_distance(&text_a, &text_b);
    let similarity = (((1.0 - distance) * 100.0) as f64).round();

    let (verdict, verdict_color) = if distance < 0.05 {
        ("✅ The files are almost identical.", "green")
    } else if distance < 0.25 {
        ("👍 The files are very similar.", "bright_green")
    } else if distance < 0.5 {
        ("🟡 The files share some similarities.", "yellow")
    } else if distance < 0.75 {
        ("⚠️ The files are quite different.", "magenta")
    } else {
        ("❌ The files are very different.", "red")
    };

    println!("{}", "Result:".bold().underline());
    println!("Document distance: {} (Similarity: {}%)", format!("{:.3}", distance).cyan(), format!("{:.0}", similarity).cyan());
    println!("{}\n", verdict.color(verdict_color).bold());
    Ok(())
}
