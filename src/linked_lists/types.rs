use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::error::Error as StdError;

pub type Result<T,E> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error<T> {
    EmptyList,
    ValueNotFound { value: T },
    IndexOutOfBounds { index: usize, max_index: usize },
    External(Box<dyn StdError + Send + Sync>),
}

impl<T: Debug> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::EmptyList => write!(f, "Operation failed: List is empty"),
            Error::ValueNotFound { value } => {
                write!(f, "Operation failed: Value {:?} not found in list", value)
            }
            Error::IndexOutOfBounds { index, max_index } => {
                write!(f, "Index {} is out of bounds (max: {})", index, max_index)
            }
            Error::External(e) => write!(f, "External error: {}", e),
        }
    }
}

impl<T: Debug> StdError for Error<T> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::External(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}