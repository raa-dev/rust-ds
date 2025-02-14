use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyTable,
    KeyNotFound,
    InvalidCapacity,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::EmptyTable => write!(f, "Operation failed: Table is empty"),
            Error::KeyNotFound => {
                write!(f, "Operation failed: Key not found in table")
            }
            Error::InvalidCapacity => {
                write!(f, "Operation failed: Invalid capacity")
            }
        }
    }
}
