use owo_colors::{OwoColorize, Style};
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
            "rainbow" => Self {
                key: Style::new().fg::<Red>(),
                string: Style::new().fg::<Green>(),
                number: Style::new().fg::<Yellow>(),
                boolean: Style::new().fg::<Blue>(),
                null: Style::new().fg::<Magenta>(),
                punctuation: Style::new().fg::<Cyan>(),
            },
            "ocean" => Self {
                key: Style::new().fg::<Blue>(),
                string: Style::new().fg::<Cyan>(),
                number: Style::new().fg::<BrightBlue>(),
                boolean: Style::new().fg::<BrightCyan>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<White>(),
            },
            "forest" => Self {
                key: Style::new().fg::<Green>(),
                string: Style::new().fg::<BrightGreen>(),
                number: Style::new().fg::<Yellow>(),
                boolean: Style::new().fg::<Red>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<White>(),
            },
            "pastel" => Self {
                key: Style::new().fg::<BrightMagenta>(),
                string: Style::new().fg::<BrightGreen>(),
                number: Style::new().fg::<BrightCyan>(),
                boolean: Style::new().fg::<BrightYellow>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<BrightWhite>(),
            },
            "sakura" => Self {
                key: Style::new().fg::<BrightMagenta>(),
                string: Style::new().fg::<BrightRed>(),
                number: Style::new().fg::<BrightCyan>(),
                boolean: Style::new().fg::<BrightYellow>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<BrightWhite>(),
            },
            "cyberpunk" => Self {
                key: Style::new().fg::<BrightMagenta>(),
                string: Style::new().fg::<BrightGreen>(),
                number: Style::new().fg::<BrightCyan>(),
                boolean: Style::new().fg::<BrightYellow>(),
                null: Style::new().fg::<Red>(),
                punctuation: Style::new().fg::<BrightWhite>(),
            },
            "ghibli" => Self {
                key: Style::new().fg::<Green>(),
                string: Style::new().fg::<Yellow>(),
                number: Style::new().fg::<Blue>(),
                boolean: Style::new().fg::<Red>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<White>(),
            },
            "evangelion" => Self {
                key: Style::new().fg::<BrightMagenta>(),
                string: Style::new().fg::<BrightCyan>(),
                number: Style::new().fg::<BrightBlue>(),
                boolean: Style::new().fg::<BrightRed>(),
                null: Style::new().fg::<BrightBlack>(),
                punctuation: Style::new().fg::<BrightWhite>(),
            },
            _ => return Err(JsonfizzError::Config(format!("Unknown theme '{}'. Use: default, solarized, mono, rainbow, ocean, forest, pastel, sakura, cyberpunk, ghibli, evangelion", name))),
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