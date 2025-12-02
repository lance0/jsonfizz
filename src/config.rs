use serde::Deserialize;
use std::path::PathBuf;
use crate::error::JsonfizzError;
use std::fs;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct PartialConfig {
    pub indent: Option<usize>,
    pub sort_keys: Option<bool>,
    pub max_depth: Option<usize>,
    pub max_string_length: Option<usize>,
    pub theme: Option<String>,
    pub format: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub indent: usize,
    pub sort_keys: bool,
    pub compact: bool,
    pub max_depth: Option<usize>,
    pub max_string_length: Option<usize>,
    pub get: Option<String>,
    pub theme: String,
    pub raw: bool,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            indent: 2,
            sort_keys: false,
            compact: false,
            max_depth: None,
            max_string_length: None,
            get: None,
            theme: "default".to_string(),
            raw: false,
            format: "json".to_string(),
        }
    }
}

impl Config {
    pub fn merge(cli: &crate::cli::CliArgs, partial: Option<PartialConfig>) -> Self {
        let mut config = Config {
            indent: cli.display.indent,
            sort_keys: cli.display.sort_keys,
            compact: cli.display.compact,
            max_depth: cli.display.max_depth,
            max_string_length: cli.display.max_string_length,
            get: cli.display.get.clone(),
            theme: cli.theme_args.theme.clone(),
            raw: cli.display.raw,
            format: cli.display.format.clone(),
        };
        if let Some(p) = partial {
            if let Some(v) = p.indent {
                config.indent = v;
            }
            if let Some(v) = p.sort_keys {
                config.sort_keys = v;
            }
            if let Some(v) = p.max_depth {
                config.max_depth = if v == 0 { None } else { Some(v) };
            }
            if let Some(v) = p.max_string_length {
                config.max_string_length = if v == 0 { None } else { Some(v) };
            }
            if let Some(v) = p.theme {
                config.theme = v;
            }
            if let Some(v) = p.format {
                config.format = v;
            }
        }
        config
    }
}

pub fn load_config() -> Result<Option<PartialConfig>, JsonfizzError> {
    let paths = get_config_paths();
    for path in paths.iter() {
        if path.exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| JsonfizzError::Config(format!("Failed to read {}: {}", path.display(), e)))?;
            let partial: PartialConfig = toml::from_str(&content)
                .map_err(|e| JsonfizzError::Config(format!("Failed to parse {}: {}", path.display(), e)))?;
            return Ok(Some(partial));
        }
    }
    Ok(None)
}

fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(config_dir) = dirs::config_dir() {
        paths.push(config_dir.join("jsonfizz").join("config.toml"));
    }
    if let Some(home_dir) = dirs::home_dir() {
        paths.push(home_dir.join(".config").join("jsonfizz").join("config.toml"));
        paths.push(home_dir.join(".jsonfizz.toml"));
    }
    paths
}