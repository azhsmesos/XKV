use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum XKVError {
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),

    #[fail(display = "invalid data path")]
    InvalidDataPath,

    #[fail(display = "key not found")]
    KeyNotFound,

    #[fail(display = "read the file end")]
    EOF,
}

impl From<io::Error> for XKVError {
    fn from(err: io::Error) -> XKVError {
        XKVError::IO(err)
    }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, XKVError>;
