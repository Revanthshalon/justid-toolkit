use super::error::{PaginationError, PaginationResult};

pub const DEFAULT_LIMIT: usize = 20;
pub const MAX_LIMIT: usize = 100;

#[derive(Debug, Clone, Copy)]
pub struct Limit(usize);

impl Limit {
    pub fn new(limit: Option<usize>) -> PaginationResult<Self> {
        match limit {
            Some(0) => Err(PaginationError::InvalidLimit("limit cannot be zero".into())),
            Some(l) if l > MAX_LIMIT => Ok(Self(MAX_LIMIT)),
            Some(l) => Ok(Self(l)),
            None => Ok(Self(DEFAULT_LIMIT)),
        }
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
