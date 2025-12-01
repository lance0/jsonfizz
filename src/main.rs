use clap::Parser;
use jsonfizz::cli::CliArgs;

fn main() {
    let args = CliArgs::parse();
    if let Err(e) = jsonfizz::run(args) {
        eprintln!("error: {}", e);
        std::process::exit(e.exit_code());
    }
}