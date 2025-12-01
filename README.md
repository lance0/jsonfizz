# jsonfizz

Fast, zero fuss JSON formatter and pretty printer for the terminal.

## Installation

```bash
cargo install jsonfizz
```

## Usage

```bash
curl https://api.example.com/data | jsonfizz

jsonfizz payload.json

cat big.json | jsonfizz --max-depth 3

jsonfizz response.json --get data.items[0].id

jsonfizz -c  # compact
```

## Options

See `jsonfizz --help`

## Configuration

Optional config at `$XDG_CONFIG_HOME/jsonfizz/config.toml` or `$HOME/.jsonfizz.toml`

```toml
indent = 4
sort_keys = true
theme = "solarized"
```

CLI flags override config.

## Themes

- `default`: Bright colors
- `solarized`: Muted palette
- `mono`: No colors

## License

MIT or Apache-2.0