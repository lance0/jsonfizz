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
                entries.sort_by_key(|(k, _)| k.clone());
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