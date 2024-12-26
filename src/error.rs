use rnbt::NbtError;
use serde::{de, ser};


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    NbtError(NbtError),
    IncompatibleListType,
    UnknownListType,
    NotWritingToList,
    NotWritingToCompound,
}

impl From<NbtError> for Error {
    fn from(value: NbtError) -> Self {
        Error::NbtError(value)
    }
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Error::Message(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
