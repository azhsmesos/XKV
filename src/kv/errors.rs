use std::array::TryFromSliceError;
use failure::Fail;
use std::io;
use std::string::FromUtf8Error;
use bincode::ErrorKind;

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

    #[fail(display="{}", _0)]
    SliceDecode(#[cause] TryFromSliceError),

    #[fail(display="{}", _0)]
    ReprDecode(#[cause] Box<bincode::ErrorKind>),

    #[fail(display="{}", _0)]
    StringDecode(#[cause] FromUtf8Error),
}

impl From<io::Error> for XKVError {
    fn from(err: io::Error) -> XKVError {
        XKVError::IO(err)
    }
}

impl From<TryFromSliceError> for XKVError {
    fn from(value: TryFromSliceError) -> Self {
        XKVError::SliceDecode(value)
    }
}

impl From<Box<bincode::ErrorKind>> for XKVError {
    fn from(value: Box<ErrorKind>) -> Self {
        XKVError::ReprDecode(value)
    }
}

impl From<FromUtf8Error> for XKVError {
    fn from(value: FromUtf8Error) -> XKVError {
        XKVError::StringDecode(value)
    }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, XKVError>;
