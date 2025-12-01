use clap::Parser;
use jsonfizz::cli::CliArgs;
use jsonfizz::error::JsonfizzError;

fn main() {
    let args = CliArgs::parse();
    if let Err(e) = jsonfizz::run(args) {
        eprintln!("error: {}", e);
        std::process::exit(e.exit_code());
    }
}