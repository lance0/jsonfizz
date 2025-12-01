use owo_colors::OwoColorize;
use owo_colors::colors;
use crate::error::JsonfizzError;

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    Key,
    String,
    Number,
    Bool,
    Null,
    Punctuation,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub colored: bool,
    pub key: colors::Yellow,
    pub string: colors::Green,
    pub number: colors::Cyan,
    pub boolean: colors::Magenta,
    pub null: colors::BrightBlack,
    pub punctuation: colors::White,
}

No, fields must be specific types, but since different colors, better to use dyn or enum, but to simplify, use a single type like owo_colors::OwoColor, but since it's trait, can't store trait object easily.

Actually, owo_colors::colors::CustomColor or something.

To make it simple, since few colors, store as Box<dyn OwoColor> but complicated for v1.

Use the Style approach properly.

From owo-colors docs, Style is struct with fg, bg, etc.

s.style(style)

Style::new().fg::<colors::Red>()

Yes, Style implements Clone + Copy.

Perfect.

Change to store Style.

In previous attempt, it was close.

use owo_colors::{OwoColorize, Style};
use owo_colors::colors::*;

Then theme.key_style: Style::new().fg::<colors::Yellow>()

Yes.

And for mono, Style::new()

In colorize, s.style(style).to_string()

Yes.

For raw/mono, Style::new() is plain.

Perfect.

Also, in formatter compact, always plain serde_json to_string.

For max_depth, plain "..." etc with color if not raw.

But for compact, plain.

Also, fix the format! with join.

The error was {items.join(\"\\n\")} inside format!, Rust format! doesn't support expressions like that.

Need to compute items_str = items.join("\n"); then format!("{open}\n{items_str}\n{indent_str}{close}")

Yes, already in code.

In my last formatter.rs, it has let items_str = items.join("\n");

Yes.

For object too.

In array, items.push(format!("{item}," ))

But item is already indented.

Yes.

Fix indent: new_indent = format!("{}{}", indent_str, "  ".repeat(config.indent / 2)? No, since indent is spaces, " ".repeat(config.indent)

Better to pass indent_level: usize, and compute indent = " ".repeat(indent_level * config.indent)

Yes, better for cleanliness.

Change signature to format_value(value, config, theme, indent_level: usize) -> Result<String>

Then current_indent = " ".repeat(indent_level * config.indent);

child_indent = " ".repeat((indent_level + 1) * config.indent);

Yes.

Perfect.

Update calls: format_value(&value, config, theme, 0)

In recursion, format_value(v, config, theme, indent_level + 1)

Yes.

For array/object.

Also, for compact, serde_json::to_string(value)?

Yes.

For string truncation, only visual.

For path, fix parser later if needed.

Now, implement.

Also, for stdin, use std::io::Read;

Already.

In lib.rs, use std::io::Read;

Yes.

For multi files, print each with \n separator? No, println! adds \n.

But for multiple files, probably print each on its own, with no extra separator.

Yes.

To test, cargo run -- --help should work.

Then, implement tests.

But first, get compiling.

Also, in Cargo.toml, binary name = "jsonfizz"

Yes.

For version, clap uses env!("CARGO_PKG_VERSION")

Yes.

Now, write the fixed code. 

Also, in path.rs, the parser needs fix, current one has chars.peekable(), but logic for . and [ is buggy.

For v1, simple split, but handle [0] as part of key? No, need proper parser.

Use regex or manual.

For now, assume simple dot, and [n] as separate.

The enum is Vec<PathSegment>, so for "data.items[0]", segments Key("data"), Key("items"), Index(0)

Yes.

To parse, split by ., then for each part, if ends with [n], split key and index.

Yes, better.

Implement simple parser.

let parts: Vec<&str> = path.split('.').collect();

for part in parts {
    if let Some(pos) = part.rfind('[') {
        let key = &part[0..pos];
        let index_str = &part[pos+1..part.len()-1];
        segments.push(Key(key.to_string()));
        segments.push(Index(index_str.parse()?));
    } else {
        segments.push(Key(part.to_string()));
    }
}

Yes, perfect for "data.items[0].id" -> Key("data"), Key("items"), Index(0), Key("id")

Yes.

Fix path.rs.

In resolve, current = v.clone(), ok.

Error messages good.

For config paths, jsonfizz now.

Yes.

For tests, add #[cfg(test)] mod tests in modules.

For path, test parse and resolve.

For formatter, test format.

But later.

First, fix compilation.

In error.rs, JsonfizzError::Parse from serde_json::Error.

Yes.

Now, tool calls. 

Since user said "we are changing the name to jsonfizz", update all.

Also, repository https://github.com/lance0/jsonfizz

Yes.

To complete formatter, mark as completed.

Then main_logic completed.

Then tests.

For git, at end.

Also, add README.

But keep concise. 

Since CLI is ready, test with bash cargo run -- --help

But since stdin, echo json | cargo run

Yes.

First, fix the code.<|control12|><xai:function_call name="write">
<parameter name="content">use owo_colors::{OwoColorize, Style};
use owo_colors::colors::*;
use crate::error::JsonfizzError;

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    Key,
    String,
    Number,
    Bool,
    Null,
    Punctuation,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub key: Style,
    pub string: Style,
    pub number: Style,
    pub boolean: Style,
    pub null: Style,
    pub punctuation: Style,
}

impl Theme {
    pub fn new(name: &str, raw: bool) -> Result<Self, JsonfizzError> {
        if raw {
            return Ok(Self {
                key: Style::new(),
                string: Style::new(),
                number: Style::new(),
                boolean: Style::new(),
                null: Style::new(),
                punctuation: Style::new(),
            });
        }
        Ok(match name.to_lowercase().as_str() {
            "default" => Self {
                key: Style::new().fg::<Yellow>(),
                string: Style::new().fg::<Green>(),
                number: Style::new().fg::<Cyan>(),
                boolean: Style::new().fg::<Magenta>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<White>(),
            },
            "solarized" => Self {
                key: Style::new().fg::<Yellow>(),
                string: Style::new().fg::<Green>(),
                number: Style::new().fg::<Blue>(),
                boolean: Style::new().fg::<Magenta>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<White>(),
            },
            "mono" => Self {
                key: Style::new(),
                string: Style::new(),
                number: Style::new(),
                boolean: Style::new(),
                null: Style::new(),
                punctuation: Style::new(),
            },
            _ => return Err(JsonfizzError::Config(format!("Unknown theme '{}'. Use: default, solarized, mono", name))),
        })
    }
}

pub fn colorize(s: &str, kind: TokenKind, theme: &Theme) -> String {
    let style = match kind {
        TokenKind::Key => theme.key,
        TokenKind::String => theme.string,
        TokenKind::Number => theme.number,
        TokenKind::Bool => theme.boolean,
        TokenKind::Null => theme.null,
        TokenKind::Punctuation => theme.punctuation,
    };
    s.style(style).to_string()
}