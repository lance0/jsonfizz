use serde_json::Value;
use crate::config::Config;
use crate::error::JsonfizzError;
use crate::theme::{colorize, TokenKind, Theme};

pub fn format_value(value: &Value, config: &Config, theme: &Theme, indent_level: usize) -> Result<String, JsonfizzError> {
    let current_indent = " ".repeat(indent_level * config.indent);
    let child_indent = " ".repeat((indent_level + 1) * config.indent);

    if config.compact {
        return Ok(serde_json::to_string(value).map_err(|e| JsonfizzError::Parse(e))?);
    }

    if let Some(max_d) = config.max_depth {
        if indent_level > max_d {
            return Ok(match value {
                Value::Object(_) => colorize("{…}", TokenKind::Punctuation, theme),
                Value::Array(_) => colorize("[…]", TokenKind::Punctuation, theme),
                _ => colorize("…", TokenKind::String, theme),
            });
        }
    }

    match value {
        Value::Null => Ok(colorize("null", TokenKind::Null, theme)),
        Value::Bool(b) => Ok(colorize(&b.to_string(), TokenKind::Bool, theme)),
        Value::Number(n) => Ok(colorize(&n.to_string(), TokenKind::Number, theme)),
        Value::String(s) => {
            let display = if let Some(max_len) = config.max_string_length {
                if s.len() > max_len {
                    format!("{}{}", &s[0..(max_len.saturating_sub(1))], "…")
                } else {
                    s.clone()
                }
            } else {
                s.clone()
            };
            Ok(format!("\"{}\"", colorize(&display, TokenKind::String, theme)))
        }
        Value::Array(arr) => {
            let mut items = Vec::new();
            for (i, v) in arr.iter().enumerate() {
                let item = format_value(v, config, theme, indent_level + 1)?;
                items.push(if i + 1 < arr.len() {
                    format!("{child_indent}{item},")
                } else {
                    format!("{child_indent}{item}")
                });
            }
            let items_str = items.join("\n");
            let open = colorize("[", TokenKind::Punctuation, theme);
            let close = colorize("]", TokenKind::Punctuation, theme);
            Ok(format!("{open}\n{items_str}\n{current_indent}{close}"))
        }
        Value::Object(map) => {
            let mut entries: Vec<_> = map.iter().collect();
            if config.sort_keys {
                entries.sort_by_key(|(k, _)| *k);
            }
            let mut items = Vec::new();
            for (k, v) in entries {
                let key_str = format!("\"{}\"", colorize(k, TokenKind::Key, theme));
                let colon = colorize(":", TokenKind::Punctuation, theme);
                let val = format_value(v, config, theme, indent_level + 1)?;
                items.push(format!("{child_indent}{key_str}{colon} {val}"));
            }
            let items_str = items.join(",\n");
            let open = colorize("{", TokenKind::Punctuation, theme);
            let close = colorize("}", TokenKind::Punctuation, theme);
            Ok(format!("{open}\n{items_str}\n{current_indent}{close}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::config::Config;
    use crate::theme::Theme;

    #[test]
    fn test_format_compact() {
        let value = json!({"key": "value", "number": 42});
        let config = Config {
            compact: true,
            ..Default::default()
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_value(&value, &config, &theme, 0).unwrap();
        assert_eq!(result, r#"{"key":"value","number":42}"#);
    }

    #[test]
    fn test_format_pretty() {
        let value = json!({"key": "value"});
        let config = Config {
            indent: 2,
            compact: false,
            ..Default::default()
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_value(&value, &config, &theme, 0).unwrap();
        assert!(result.contains("{\n"));
        assert!(result.contains("  \"key\": \"value\""));
        assert!(result.contains("\n}"));
    }

    #[test]
    fn test_max_depth() {
        let value = json!({"nested": {"deep": {"value": 123}}});
        let config = Config {
            max_depth: Some(2),
            ..Default::default()
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_value(&value, &config, &theme, 0).unwrap();
        assert!(result.contains("…"));
    }

    #[test]
    fn test_sort_keys() {
        let value = json!({"z": 1, "a": 2, "m": 3});
        let config = Config {
            sort_keys: true,
            ..Default::default()
        };
        let theme = Theme::new("mono", false).unwrap();
        let result = format_value(&value, &config, &theme, 0).unwrap();
        // Keys should be sorted: a, m, z
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines[1].contains("\"a\""));
        assert!(lines[2].contains("\"m\""));
        assert!(lines[3].contains("\"z\""));
    }
}