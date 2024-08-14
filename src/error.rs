use core::fmt;
use std::fmt::Display;
use std::io;
use std::result;
use std::str::Utf8Error;

pub enum ErrorType {
    IO,
    Serialization,
}

#[derive(Debug)]
pub enum DocError {
    IO(io::Error),
    Serialization(String),
    Deserialization(String),
}

impl DocError {
    pub fn get_type(&self) -> ErrorType {
        match self {
            DocError::IO(_) => ErrorType::IO,
            _ => ErrorType::Serialization,
        }
    }
}

impl Display for DocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocError::IO(err) => fmt::Display::fmt(err, f),
            DocError::Serialization(err) => f.write_str(&format!("Serialization err: {}", err)),
            DocError::Deserialization(err) => f.write_str(&format!("Deserialization err: {}", err)),
        }
    }
}

/// Alias for a `Result` with the error type [Error].
pub type Result<T> = result::Result<T, DocError>;

impl From<serde_json::Error> for DocError {
    fn from(value: serde_json::Error) -> Self {
        DocError::Serialization(value.to_string())
    }
}

impl From<Utf8Error> for DocError {
    fn from(value: Utf8Error) -> Self {
        DocError::Deserialization(value.to_string())
    }
}

impl From<io::Error> for DocError {
    fn from(value: io::Error) -> Self {
        DocError::IO(value)
    }
}
