use std::num::ParseIntError;

#[derive(Debug)]
pub enum PaginationError {
    InvalidLimit(String),
    InvalidPage(String),
    InvalidToken(String),
    InvalidKey(String),
    ParseError(ParseIntError),
    MissingParameter(String),
}

pub type PaginationResult<T> = Result<T, PaginationError>;

impl std::fmt::Display for PaginationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaginationError::InvalidLimit(details) => write!(f, "invalid limit value: {}", details),
            PaginationError::InvalidPage(details) => write!(f, "invalid page value: {}", details),
            PaginationError::InvalidToken(details) => write!(f, "invalid token: {}", details),
            PaginationError::InvalidKey(details) => write!(f, "invalid key: {}", details),
            PaginationError::ParseError(details) => write!(f, "parse error: {}", details),
            PaginationError::MissingParameter(details) => {
                write!(f, "missing required parameter: {}", details)
            }
        }
    }
}

impl std::error::Error for PaginationError {}

impl From<ParseIntError> for PaginationError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}
