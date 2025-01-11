use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyList,
    ValueNotFound,
    IndexOutOfBounds,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::EmptyList => write!(f, "Operation failed: List is empty"),
            Error::ValueNotFound => {
                write!(f, "Operation failed: Value not found in list")
            }
            Error::IndexOutOfBounds => {
                write!(f, "Index is out of bounds")
            }
        }
    }
}
