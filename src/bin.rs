use clap::Parser;
use sbom_rs::cli::{
    opts::{Cli, Command},
    scan::run_scan,
};

fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(cli.verbosity.to_level())
        .init();

    match cli.cmd {
        Command::Scan(args) => run_scan(args),
    }
}
