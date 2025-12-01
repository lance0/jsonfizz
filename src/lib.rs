pub mod cli;
pub mod config;
pub mod error;
pub mod formatter;
pub mod path;
pub mod theme;

pub use error::JsonfizzError;

use std::io::{self, Read};

pub fn run(args: cli::CliArgs) -> Result<(), JsonfizzError> {
    let config = args.to_config();
    let theme = crate::theme::Theme::new(&config.theme, config.raw)?;
    process_inputs(&args.files, &config, &theme)
}

fn process_inputs(files: &[String], config: &crate::config::Config, theme: &crate::theme::Theme) -> Result<(), JsonfizzError> {
    if files.is_empty() {
        let mut input = String::new();
        io::stdin().lock().read_to_string(&mut input)?;
        let value: serde_json::Value = serde_json::from_str(&input)?;
        let value = apply_get(&value, &config.get)?;
        let output = crate::formatter::format_value(&value, config, theme, 0)?;
        println!("{}", output);
    } else {
        for file in files {
            let input = if file == "-" {
                let mut s = String::new();
                io::stdin().lock().read_to_string(&mut s)?;
                s
            } else {
                std::fs::read_to_string(file)?
            };
            let value: serde_json::Value = serde_json::from_str(&input)?;
            let value = apply_get(&value, &config.get)?;
            let output = crate::formatter::format_value(&value, config, theme, 0)?;
            println!("{}", output);
        }
    }
    Ok(())
}

fn apply_get(value: &serde_json::Value, get_path: &Option<String>) -> Result<serde_json::Value, JsonfizzError> {
    if let Some(path_str) = get_path.as_deref() {
        let path = crate::path::parse_path(path_str)?;
        crate::path::resolve(value, &path)
    } else {
        Ok(value.clone())
    }
}