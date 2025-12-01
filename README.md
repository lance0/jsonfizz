# 🎨 jsonfizz

[![Crates.io](https://img.shields.io/crates/v/jsonfizz.svg)](https://crates.io/crates/jsonfizz)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/lance0/jsonfizz)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/actions/workflow/status/lance0/jsonfizz/ci.yml)](https://github.com/lance0/jsonfizz/actions)

Fast, zero fuss JSON formatter and pretty printer for the terminal. ✨

## ✨ Features

- 🚀 **Blazing fast** - Written in Rust
- 🎨 **Beautiful themes** - Multiple color schemes
- 🔍 **JSON path queries** - Extract specific values
- 📏 **Depth limiting** - Handle large JSON gracefully
- 🎯 **Multiple inputs** - Files, stdin, or pipes
- ⚙️ **Configurable** - TOML config file support

## 📦 Installation

```bash
cargo install jsonfizz
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

Create `~/.jsonfizz.toml`:

```toml
indent = 4
sort_keys = true
theme = "rainbow"
```

CLI flags override config.

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
      --theme <THEME>                          Color theme: default, solarized, mono, rainbow, ocean, forest, pastel, sakura, cyberpunk, ghibli, evangelion [default: default]
      --generate-completion <SHELL>            Generate shell completion script [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                                   Print help
  -V, --version                                Print version
```
Usage: jsonfizz [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Input files (use - for stdin)

Options:
  -i, --indent <INDENT>                        Indentation spaces [default: 2]
      --sort-keys                              Sort object keys alphabetically
  -c, --compact                                Compact (minified) output
      --max-depth <MAX_DEPTH>                  Limit nesting depth
      --max-string-length <MAX_STRING_LENGTH>  Truncate long strings
      --get <GET>                              JSON path query (e.g., data.items[0])
      --raw                                    Disable colors
      --theme <THEME>                          Color theme (see available themes below) [default: default]
  -h, --help                                   Print help
  -V, --version                                Print version
```

## 🤝 Contributing

PRs welcome! Please format with `cargo fmt` and test with `cargo test`.

## 📄 License

Copyright © 2025 Lance. Licensed under MIT or Apache-2.0.