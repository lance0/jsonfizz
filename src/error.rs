use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonfizzError {
    #[error("Invalid JSON: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Path error: {0}")]
    Path(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl JsonfizzError {
    pub fn exit_code(&self) -> i32 {
        match self {
            JsonfizzError::Parse(_) | JsonfizzError::Path(_) => 1,
            _ => 2,
        }
    }
}