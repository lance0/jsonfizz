use clap::Parser;
use crate::config::{Config, load_config};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "🎨 Fast, zero fuss JSON formatter and pretty printer for the terminal",
    long_about = None
)]
pub struct CliArgs {
    /// Input files (use - for stdin)
    #[arg(name = "FILE", num_args = 0..)]
    pub files: Vec<String>,

    #[clap(flatten)]
    pub display: DisplayArgs,

    #[clap(flatten)]
    pub theme_args: ThemeArgs,

    #[arg(long, value_enum, help = "Generate shell completion script")]
    pub generate_completion: Option<clap_complete::Shell>,

    #[arg(long, help = "Run performance benchmarks")]
    pub benchmark: bool,
}

#[derive(clap::Args, Debug, Clone)]
pub struct DisplayArgs {
    #[arg(short = 'i', long, default_value_t = 2)]
    pub indent: usize,

    #[arg(long, default_value_t = true)]
    pub sort_keys: bool,

    #[arg(short = 'c', long)]
    pub compact: bool,

    #[arg(long)]
    pub max_depth: Option<usize>,

    #[arg(long)]
    pub max_string_length: Option<usize>,

    #[arg(long)]
    pub get: Option<String>,

    #[arg(long)]
    pub raw: bool,

    #[arg(long, default_value = "json", help = "Output format: json, yaml, toml")]
    pub format: String,

    #[arg(long, help = "Input format: json, toml [default: json]")]
    pub input_format: Option<String>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct ThemeArgs {
    #[arg(long, default_value = "default", help = "Color theme: default, solarized, mono, rainbow, ocean, forest, pastel, sakura, cyberpunk, ghibli, evangelion")]
    pub theme: String,
}



impl CliArgs {
    pub fn to_config(&self) -> Config {
        match load_config() {
            Ok(partial) => Config::merge(self, partial),
            Err(e) => {
                eprintln!("Config load error: {}", e);
                Config::merge(self, None)
            }
        }
    }
}