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
        // For stdin, read efficiently and warn about large inputs
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buffer = Vec::new();

        // Read in chunks to avoid memory issues with extremely large inputs
        let mut chunk = [0; 8192]; // 8KB chunks
        loop {
            let bytes_read = reader.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..bytes_read]);

            // Warn if input is getting very large (>10MB)
            if buffer.len() > 10 * 1024 * 1024 {
                eprintln!("Warning: Large input detected ({} MB). Consider using --max-depth or processing in chunks.", buffer.len() / (1024 * 1024));
            }
        }

        // Try to parse as JSON
        let input_str = std::str::from_utf8(&buffer)
            .map_err(|e| JsonfizzError::Yaml(format!("Invalid UTF-8 in input: {}", e)))?;
        let value: serde_json::Value = serde_json::from_str(input_str)?;
        let value = apply_get(&value, &config.get)?;
        let output = format_output(&value, config, theme)?;
        println!("{}", output);
    } else {
        for file in files {
            let input = if file == "-" {
                let stdin = io::stdin();
                let mut reader = stdin.lock();
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer)?;
                String::from_utf8(buffer)
                    .map_err(|e| JsonfizzError::Yaml(format!("Invalid UTF-8 in stdin: {}", e)))?
            } else {
                // Check file size before reading
                let metadata = std::fs::metadata(file)?;
                let file_size = metadata.len();

                if file_size > 50 * 1024 * 1024 { // 50MB
                    eprintln!("Warning: Large file detected ({} MB): {}. Consider using --max-depth for better performance.", file_size / (1024 * 1024), file);
                }

                std::fs::read_to_string(file)?
            };
            let value: serde_json::Value = serde_json::from_str(&input)?;
            let value = apply_get(&value, &config.get)?;
            let output = format_output(&value, config, theme)?;
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

fn format_output(value: &serde_json::Value, config: &crate::config::Config, theme: &crate::theme::Theme) -> Result<String, JsonfizzError> {
    match config.format.as_str() {
        "json" => crate::formatter::format_value(value, config, theme, 0),
        "yaml" => {
            let yaml = serde_yaml::to_string(value)
                .map_err(|e| JsonfizzError::Yaml(format!("YAML serialization error: {}", e)))?;
            Ok(yaml)
        }
        _ => Err(JsonfizzError::Config(format!("Unsupported format: {}. Supported: json, yaml", config.format))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::config::Config;
    use crate::theme::Theme;

    #[test]
    fn test_format_yaml() {
        let value = json!({"name": "test", "version": 1.0});
        let config = Config {
            format: "yaml".to_string(),
            ..Default::default()
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_output(&value, &config, &theme).unwrap();
        assert!(result.contains("name: test"));
        assert!(result.contains("version: 1.0"));
    }

    #[test]
    fn test_format_unsupported() {
        let value = json!({"test": "value"});
        let config = Config {
            format: "xml".to_string(),
            ..Default::default()
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_output(&value, &config, &theme);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported format"));
    }
}