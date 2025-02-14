use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyTable,
    ValueNotFound,
    IndexOutOfBounds,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::EmptyTable => write!(f, "Operation failed: Table is empty"),
            Error::ValueNotFound => {
                write!(f, "Operation failed: Value not found in table")
            }
            Error::IndexOutOfBounds => {
                write!(f, "Index is out of bounds")
            }
        }
    }
}
