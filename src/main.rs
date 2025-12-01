use clap::{Parser, CommandFactory};
use clap_complete::generate;
use jsonfizz::cli::CliArgs;

fn main() {
    let args = CliArgs::parse();

    // Handle completion generation
    if let Some(shell) = args.generate_completion {
        let mut cmd = CliArgs::command();
        let bin_name = cmd.get_name().to_string();
        generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        return;
    }

    if let Err(e) = jsonfizz::run(args) {
        eprintln!("error: {}", e);
        std::process::exit(e.exit_code());
    }
}