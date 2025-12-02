# 🎨 jsonfizz

[![Crates.io](https://img.shields.io/crates/v/jsonfizz.svg)](https://crates.io/crates/jsonfizz)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/lance0/jsonfizz)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

Fast, zero fuss JSON formatter and pretty printer for the terminal. ✨

## ✨ Features

- 🚀 **Blazing fast** - Written in Rust
- 🎨 **Beautiful themes** - 11 color schemes including anime themes
- 🔍 **JSON path queries** - Extract specific values
- 📏 **Depth limiting** - Handle large JSON gracefully
- 🎯 **Multiple inputs** - Files, stdin, or pipes
- ⚙️ **Configurable** - TOML config file support
- 📄 **Format conversion** - JSON ↔ YAML ↔ TOML, CSV output
- 📑 **CSV friendly** - Read CSV and convert to JSON
- 📄 **Multiple formats** - JSON and YAML output
- ✅ **Schema checks** - Optional JSON Schema validation
- 🐚 **Shell completion** - Auto-completion for bash/zsh/fish
- 📊 **Performance benchmarks** - Built-in performance testing
- 📈 **Progress indicators** - Feedback for large file processing

## 📦 Installation

```bash
cargo install jsonfizz
```

## 📚 Library Usage

You can use `jsonfizz` as a Rust library in your own projects:

```toml
[dependencies]
jsonfizz = "0.2.0"
serde_json = "1.0"
```

```rust
use jsonfizz::{config::Config, theme::Theme, formatter::format_value};
use serde_json::json;

fn main() {
    let value = json!({"name": "jsonfizz", "awesome": true});
    
    // Configure formatting options
    let config = Config {
        indent: 4,
        theme: "ocean".to_string(),
        ..Config::default()
    };
    
    // Initialize theme
    let theme = Theme::new(&config.theme, false).unwrap();
    
    // Format the value
    let formatted = format_value(&value, &config, &theme, 0).unwrap();
    println!("{}", formatted);
}
```

## 🚀 Usage

### Basic formatting
```bash
# From API
curl https://api.github.com/users/octocat | jsonfizz

# From file
jsonfizz data.json

# From stdin
cat large.json | jsonfizz
```

### Advanced features
```bash
# Watch file for changes and reformat on the fly
jsonfizz data.json --watch

# Extract specific values with JSON path
jsonfizz response.json --get data.items[0].name

# Limit depth for large files
cat huge.json | jsonfizz --max-depth 3

# Compact output
jsonfizz data.json --compact

# Custom indentation
jsonfizz data.json --indent 4

# Sort object keys
jsonfizz data.json --sort-keys

# Format conversion (JSON ↔ YAML ↔ TOML ↔ CSV)
# Read TOML, output as JSON
jsonfizz config.toml --input-format toml --format json

# Read YAML, output as TOML
echo 'name: test' | jsonfizz --input-format yaml --format toml

# Convert JSON array to CSV
echo '[{"name":"Alice","age":30},{"name":"Bob","age":25}]' | jsonfizz --format csv

# Read CSV and output JSON
jsonfizz data.csv --input-format csv --format json

# Validate against a JSON Schema
jsonfizz data.json --schema schema.json

# Control color output
jsonfizz data.json --color never    # Never use colors
jsonfizz data.json --color always   # Always use colors
jsonfizz data.json --color auto     # Auto-detect (default)

# Read JSON, output as YAML
jsonfizz data.json --format yaml

# Read JSON, output as TOML
jsonfizz data.json --format toml

# Run performance benchmarks
jsonfizz --benchmark
```

### Color themes
```bash
# Rainbow theme 🌈
jsonfizz data.json --theme rainbow

# Ocean theme 🌊
jsonfizz data.json --theme ocean

# Forest theme 🌲
jsonfizz data.json --theme forest

# Pastel theme 🎨
jsonfizz data.json --theme pastel

# Anime themes 🌸🤖🏔️👁️
jsonfizz data.json --theme sakura
jsonfizz data.json --theme cyberpunk
jsonfizz data.json --theme ghibli
jsonfizz data.json --theme evangelion

# Generate shell completions
jsonfizz --generate-completion bash > ~/.bash_completion.d/jsonfizz
jsonfizz --generate-completion zsh > ~/.zsh/_jsonfizz
jsonfizz --generate-completion fish > ~/.config/fish/completions/jsonfizz.fish
```

## ⚙️ Configuration

Create `~/.config/jsonfizz/config.toml` or `~/.jsonfizz.toml` to set persistent defaults.

**Full Example:**

```toml
# Indentation size (spaces)
indent = 2

# Sort keys alphabetically (true/false)
sort_keys = true

# Default color theme
theme = "ocean"

# Default output format (json, yaml, toml, csv)
format = "json"

# Max depth to recurse (0 = unlimited)
max_depth = 0

# Max string length before truncation (0 = unlimited)
max_string_length = 0

# Optional: Path to a default JSON schema for validation
# schema = "/path/to/schema.json"
```

CLI flags override config.

## 🔍 JSON Path Syntax

The `--get` flag supports a simple dot-notation syntax for extracting values:

- `key`: Access a property of an object.
- `array[index]`: Access an element of an array.
- `data.items[0].name`: Nested access.

**Examples:**
- `users[0].id`
- `config.server.port`
- `rows[5]`

## ✅ Schema Validation

Validate your JSON against a standard [JSON Schema](https://json-schema.org/).

```bash
jsonfizz data.json --schema schema.json
```

If validation fails, `jsonfizz` will print a clear error message indicating the location of the violation and exit with code 1.

## ❓ Troubleshooting

**"Error: UTF-8"**
`jsonfizz` currently only supports valid UTF-8 input. Ensure your files are encoded correctly.

**"Watch limit reached"**
If using `--watch` on Linux, you might hit the system's file watcher limit. Increase it with:
`sysctl fs.inotify.max_user_watches=524288`


## 🎨 Themes

| Theme | Description | Preview |
|-------|-------------|---------|
| `default` | Bright, balanced colors | Keys: 🟡 Strings: 🟢 Numbers: 🔵 |
| `solarized` | Muted, eye-friendly | Keys: 🟡 Strings: 🟢 Numbers: 🔵 |
| `mono` | No colors | Plain text |
| `rainbow` 🌈 | Vibrant rainbow | Keys: 🔴 Strings: 🟢 Numbers: 🟡 |
| `ocean` 🌊 | Cool blue tones | Keys: 🔵 Strings: 🔵 Numbers: 🔵 |
| `forest` 🌲 | Nature greens | Keys: 🟢 Strings: 🟢 Numbers: 🟡 |
| `pastel` 🎨 | Soft pastels | Keys: 🩷 Strings: 🩷 Numbers: 🩷 |
| `sakura` 🌸 | Anime pink | Keys: 🩷 Strings: 🔴 Numbers: 🔵 |
| `cyberpunk` 🤖 | Neon cyber | Keys: 🩷 Strings: 🟢 Numbers: 🔵 |
| `ghibli` 🏔️ | Studio Ghibli | Keys: 🟢 Strings: 🟡 Numbers: 🔵 |
| `evangelion` 👁️ | Purple & teal | Keys: 🩷 Strings: 🔵 Numbers: 🔵 |

## 📋 Options

```
Usage: jsonfizz [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Input files (use - for stdin)

Options:
  -i, --indent <INDENT>                        [default: 2]
      --sort-keys
  -c, --compact
      --max-depth <MAX_DEPTH>
      --max-string-length <MAX_STRING_LENGTH>
      --get <GET>
      --raw
      --format <FORMAT>                        Output format: json, yaml, toml, csv [default: json]
      --input-format <INPUT_FORMAT>            Input format: json, yaml, toml, csv [default: json]
      --schema <SCHEMA>                        Path to a JSON Schema file for validation
      --color <COLOR>                          Color output control: auto, always, never [default: auto]
      --theme <THEME>                          Color theme (see available themes below) [default: default]
  -h, --help                                   Print help
  -V, --version                                Print version
```

## 🤝 Contributing

PRs welcome! Please format with `cargo fmt` and test with `cargo test`.

## 📄 License

Copyright © 2025 Lance. Licensed under MIT or Apache-2.0.
