use std::num::TryFromIntError;

use rnbt::NbtError;
use serde::{de, ser};


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    IoError(std::io::Error),
    Uft8Error(std::string::FromUtf8Error),
    IntError(TryFromIntError),
    IncompatibleListType,
    UnknownListType,
    NotWritingToList,
    NotWritingToCompound,
    ExpectedByte,
    ExpectedShort,
    ExpectedByteArray,
    ExpectedLong,
    ExpectedCompound,
    ExpectedStruct(String),
    ExpectedIdentifier,
    ExpectedEnd,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::IntError(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Error::Uft8Error(value)
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
