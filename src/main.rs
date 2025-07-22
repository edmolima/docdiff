mod cli;
mod utils;

use anyhow::Result;
use docdiff::algorithm;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        Some(("diff", sub_m)) => {
            let original = sub_m.get_one::<String>("original").expect("required");
            let modified = sub_m.get_one::<String>("modified").expect("required");

            println!("Comparing '{original}' with '{modified}'...");

            let text_a = utils::read_file(original)?;
            let text_b = utils::read_file(modified)?;

            let pb = indicatif::ProgressBar::new(100);
            pb.inc(100);
            pb.finish_with_message("Comparison finished!");

            let distance = algorithm::document_distance::document_distance(&text_a, &text_b);
            println!("Document distance: {distance:.3}");
            Ok(())
        }
        Some(("info", _)) => {
            println!("docdiff - Compare two files using document distance algorithms");
            Ok(())
        }
        _ => {
            cli::print_help();
            Ok(())
        }
    }
}
