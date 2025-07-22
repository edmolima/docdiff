use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("docdiff")
        .about("Compare two files using document distance algorithms")
        .subcommand(
            Command::new("diff")
                .about("Compare two text files")
                .arg(Arg::new("original").required(true).help("Path to the original file"))
                .arg(Arg::new("modified").required(true).help("Path to the modified file"))
        )
        .subcommand(
            Command::new("info")
                .about("Show information about the program")
        )
}

pub fn print_help() {
    build_cli().print_help().unwrap();
}
