use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonfizzError {
    #[error("{format} parse error{loc}: {message}")]
    Parse {
        format: &'static str,
        message: String,
        loc: String,
    },

    #[error("Path error: {0}")]
    Path(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Error: {0}")]
    Data(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl JsonfizzError {
    pub fn exit_code(&self) -> i32 {
        match self {
            JsonfizzError::Parse { .. } | JsonfizzError::Path(_) => 1,
            _ => 2,
        }
    }

    pub fn parse_error(format: &'static str, message: impl Into<String>, line: Option<usize>, column: Option<usize>) -> Self {
        let loc = match (line, column) {
            (Some(l), Some(c)) => format!(" at line {}, column {}", l, c),
            (Some(l), None) => format!(" at line {}", l),
            (None, Some(c)) => format!(" at column {}", c),
            (None, None) => String::new(),
        };
        JsonfizzError::Parse {
            format,
            message: message.into(),
            loc,
        }
    }
}
