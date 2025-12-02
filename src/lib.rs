pub mod cli;
pub mod config;
pub mod error;
pub mod formatter;
pub mod path;
pub mod theme;

pub use error::JsonfizzError;

use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Instant;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub fn run<W: Write>(args: cli::CliArgs, mut writer: W) -> Result<(), JsonfizzError> {
    let config = args.to_config();

    // Determine if colors should be used
    let use_colors = match config.color {
        Some(cli::ColorChoice::Always) => true,
        Some(cli::ColorChoice::Never) => false,
        Some(cli::ColorChoice::Auto) | None => {
            // Auto-detect: use colors if stdout is TTY and NO_COLOR is not set
            std::env::var("NO_COLOR").is_err() && atty::is(atty::Stream::Stdout)
        }
    };

    let theme = crate::theme::Theme::new(&config.theme, config.raw || !use_colors)?;
    process_inputs(&args.files, &config, &theme, &mut writer)
}

fn process_inputs<W: Write>(files: &[String], config: &crate::config::Config, theme: &crate::theme::Theme, writer: &mut W) -> Result<(), JsonfizzError> {
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

        // Try to parse based on input format
        let input_str = std::str::from_utf8(&buffer)
            .map_err(|e| JsonfizzError::Yaml(format!("Invalid UTF-8 in input: {}", e)))?;
        let value: serde_json::Value = parse_input(input_str, &config.input_format)?;
        let value = apply_get(&value, &config.get)?;
        let output = format_output(&value, config, theme)?;
        writeln!(writer, "{}", output)?;
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
            let value: serde_json::Value = parse_input(&input, &config.input_format)?;
            let value = apply_get(&value, &config.get)?;
            let output = format_output(&value, config, theme)?;
            writeln!(writer, "{}", output)?;
        }
    }
    Ok(())
}

fn parse_input(input: &str, format: &str) -> Result<serde_json::Value, JsonfizzError> {
    match format {
        "json" => serde_json::from_str(input)
            .map_err(|e| JsonfizzError::Parse(e)),
        "yaml" => serde_yaml::from_str(input)
            .map_err(|e| JsonfizzError::Yaml(format!("YAML parse error: {}", e))),
        "toml" => {
            let toml_value: toml::Value = toml::from_str(input)
                .map_err(|e| JsonfizzError::Yaml(format!("TOML parse error: {}", e)))?;
            // Convert TOML to JSON Value
            serde_json::to_value(toml_value)
                .map_err(|e| JsonfizzError::Yaml(format!("TOML to JSON conversion error: {}", e)))
        }
        _ => Err(JsonfizzError::Config(format!("Unsupported input format: {}. Supported: json, yaml, toml", format))),
    }
}

fn apply_get(value: &serde_json::Value, get_path: &Option<String>) -> Result<serde_json::Value, JsonfizzError> {
    if let Some(path_str) = get_path.as_deref() {
        let path = crate::path::parse_path(path_str)?;
        crate::path::resolve(value, &path)
    } else {
        Ok(value.clone())
    }
}

fn convert_to_csv(value: &serde_json::Value) -> Result<String, JsonfizzError> {
    match value {
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(String::new());
            }

            // Collect all unique keys from all objects
            let mut all_keys = std::collections::BTreeSet::new();
            for item in arr {
                if let serde_json::Value::Object(obj) = item {
                    for key in obj.keys() {
                        all_keys.insert(key.clone());
                    }
                }
            }

            let keys: Vec<String> = all_keys.into_iter().collect();

            // Create CSV writer
            let mut wtr = csv::WriterBuilder::new()
                .has_headers(true)
                .from_writer(vec![]);

            // Write headers
            wtr.write_record(&keys)
                .map_err(|e| JsonfizzError::Yaml(format!("CSV header write error: {}", e)))?;

            // Write data rows
            for item in arr {
                if let serde_json::Value::Object(obj) = item {
                    let mut row = Vec::new();
                    for key in &keys {
                        let value = obj.get(key)
                            .and_then(|v| match v {
                                serde_json::Value::String(s) => Some(s.clone()),
                                serde_json::Value::Number(n) => Some(n.to_string()),
                                serde_json::Value::Bool(b) => Some(b.to_string()),
                                serde_json::Value::Null => Some(String::new()),
                                _ => Some(v.to_string()),
                            })
                            .unwrap_or_default();
                        row.push(value);
                    }
                    wtr.write_record(&row)
                        .map_err(|e| JsonfizzError::Yaml(format!("CSV row write error: {}", e)))?;
                }
            }

            let csv_data = wtr.into_inner()
                .map_err(|e| JsonfizzError::Yaml(format!("CSV writer error: {}", e)))?;
            String::from_utf8(csv_data)
                .map_err(|e| JsonfizzError::Yaml(format!("CSV encoding error: {}", e)))
        }
        _ => Err(JsonfizzError::Yaml("CSV output requires a JSON array of objects".to_string())),
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
        "toml" => {
            let toml = toml::to_string(value)
                .map_err(|e| JsonfizzError::Yaml(format!("TOML serialization error: {}", e)))?;
            Ok(toml)
        }
        "csv" => {
            convert_to_csv(value)
        }
        _ => Err(JsonfizzError::Config(format!("Unsupported format: {}. Supported: json, yaml, toml, csv", config.format))),
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

pub fn run_watch(path: String, args: cli::CliArgs) -> Result<(), JsonfizzError> {
    let config = args.to_config();

    // Initial format
    println!("🔄 Initial format of {}", path);
    if let Err(e) = process_file(&path, &config) {
        eprintln!("Initial format error: {}", e);
    }

    // Setup watcher
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        // Forward events through the channel for processing in the main loop
        if tx.send(res).is_err() {
            eprintln!("Watch channel closed unexpectedly");
        }
    }).map_err(|e| JsonfizzError::Config(format!("Failed to start watcher: {}", e)))?;
    watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)
        .map_err(|e| JsonfizzError::Config(format!("Failed to watch {}: {}", path, e)))?;

    println!("👀 Watching {} for changes (Ctrl+C to exit)...", path);

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        println!("\n🔄 File changed, reformatting...");
                        if let Err(e) = process_file(&path, &config) {
                            eprintln!("Reformat error: {}", e);
                        }
                    }
                    EventKind::Remove(_) => {
                        eprintln!("File removed; stopping watcher.");
                        break;
                    }
                    _ => {}
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch event error: {:?}", e);
            }
            Err(e) => {
                eprintln!("Watch channel error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn process_file(path: &str, config: &crate::config::Config) -> Result<(), JsonfizzError> {
    let input = std::fs::read_to_string(path)?;
    let value: serde_json::Value = parse_input(&input, &config.input_format)?;
    let value = apply_get(&value, &config.get)?;
    let use_colors = match config.color {
        Some(cli::ColorChoice::Always) => true,
        Some(cli::ColorChoice::Never) => false,
        Some(cli::ColorChoice::Auto) | None => atty::is(atty::Stream::Stdout),
    };
    let theme = crate::theme::Theme::new(&config.theme, config.raw || !use_colors)?;
    let output = format_output(&value, config, &theme)?;
    println!("--- file updated ---");
    println!("{}", output);
    println!("");
    Ok(())
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
            input_format: "json".to_string(),
            color: None,
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_output(&value, &config, &theme).unwrap();
        assert!(result.contains("name: test"));
        assert!(result.contains("version: 1.0"));
    }

    #[test]
    fn test_format_toml() {
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
            format: "toml".to_string(),
            input_format: "json".to_string(),
            color: None,
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_output(&value, &config, &theme).unwrap();
        assert!(result.contains("name = \"test\""));
        assert!(result.contains("version = 1.0"));
    }

    #[test]
    fn test_parse_toml_input() {
        let toml_input = r#"name = "test"
version = 1.0"#;
        let value = parse_input(toml_input, "toml").unwrap();
        assert_eq!(value["name"], "test");
        assert_eq!(value["version"], 1.0);
    }

    #[test]
    fn test_parse_yaml_input() {
        let yaml_input = r#"name: test
version: 1.0"#;
        let value = parse_input(yaml_input, "yaml").unwrap();
        assert_eq!(value["name"], "test");
        assert_eq!(value["version"], 1.0);
    }

    #[test]
    fn test_format_csv() {
        let value = json!([
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"}
        ]);
        let config = Config {
            indent: 2,
            sort_keys: false,
            compact: false,
            max_depth: None,
            max_string_length: None,
            get: None,
            theme: "mono".to_string(),
            raw: false,
            format: "csv".to_string(),
            input_format: "json".to_string(),
            color: None,
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_output(&value, &config, &theme).unwrap();
        assert!(result.contains("age,city,name"));
        assert!(result.contains("30,NYC,Alice"));
        assert!(result.contains("25,LA,Bob"));
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
