pub mod cli;
pub mod config;
pub mod error;
pub mod formatter;
pub mod path;
pub mod theme;

pub use error::JsonfizzError;

use std::io::{self, Read};

use std::time::Instant;

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
        let start_time = Instant::now();

        // Read in chunks to avoid memory issues with extremely large inputs
        let mut chunk = [0; 8192]; // 8KB chunks
        let mut total_bytes = 0;
        let mut show_progress = false;

        loop {
            let bytes_read = reader.read(&mut chunk)?;
            if bytes_read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..bytes_read]);
            total_bytes += bytes_read;

            // Show progress for large inputs (>5MB)
            if total_bytes > 5 * 1024 * 1024 && !show_progress {
                show_progress = true;
                eprintln!("Reading large input from stdin... ({} MB)", total_bytes / (1024 * 1024));
            }

            // Warn if input is getting very large (>50MB)
            if total_bytes > 50 * 1024 * 1024 && start_time.elapsed().as_secs() > 5 {
                eprintln!("Warning: Very large input detected ({} MB). Processing may be slow.", total_bytes / (1024 * 1024));
            }
        }

        if show_progress {
            eprintln!("Read {} MB in {:.2}s", total_bytes / (1024 * 1024), start_time.elapsed().as_secs_f32());
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

pub fn run_benchmarks() {
    println!("🚀 Running jsonfizz performance benchmarks...\n");

    // Benchmark 1: Small JSON formatting
    let small_json = r#"{"name":"test","value":42,"items":[1,2,3]}"#;
    let start = Instant::now();
    for _ in 0..1000 {
        let _: serde_json::Value = serde_json::from_str(small_json).unwrap();
    }
    let parse_time = start.elapsed();

    let start = Instant::now();
    for _ in 0..1000 {
        let value: serde_json::Value = serde_json::from_str(small_json).unwrap();
        let mut config = crate::config::Config::default();
        config.theme = "mono".to_string();
        let theme = crate::theme::Theme::new("mono", false).unwrap();
        format_output(&value, &config, &theme).unwrap();
    }
    let format_time = start.elapsed();

    println!("📊 Small JSON ({} chars):", small_json.len());
    println!("  Parse (1000x): {:.2}ms", parse_time.as_millis());
    println!("  Format (1000x): {:.2}ms", format_time.as_millis());
    println!("  Total: {:.2}ms", (parse_time + format_time).as_millis());

    // Benchmark 3: YAML vs JSON output
    let test_value: serde_json::Value = serde_json::from_str(r#"{"test":"data","array":[1,2,3]}"#).unwrap();
    let test_theme = crate::theme::Theme::new("default", false).unwrap();

    let start = Instant::now();
    let mut config_json = crate::config::Config::default();
    config_json.format = "json".to_string();
    format_output(&test_value, &config_json, &test_theme).unwrap();
    let json_time = start.elapsed();

    let start = Instant::now();
    let mut config_yaml = crate::config::Config::default();
    config_yaml.format = "yaml".to_string();
    format_output(&test_value, &config_yaml, &test_theme).unwrap();
    let yaml_time = start.elapsed();

    println!("\n📊 Format comparison:");
    println!("  JSON: {:.2}ms", json_time.as_millis());
    println!("  YAML: {:.2}ms", yaml_time.as_millis());
    println!("  Ratio: {:.2}x", yaml_time.as_secs_f64() / json_time.as_secs_f64());

    println!("\n✅ Benchmarks complete!");
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
            indent: 2,
            sort_keys: false,
            compact: false,
            max_depth: None,
            max_string_length: None,
            get: None,
            theme: "mono".to_string(),
            raw: false,
            format: "yaml".to_string(),
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