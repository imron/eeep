use std::array::TryFromSliceError;

#[derive(Debug)]
pub enum Error {
    InvalidInputString,
    InvalidTimestamp,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::InvalidInputString => None,
            Error::InvalidTimestamp => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::InvalidInputString => {
                write!(f, "Invalid input string")
            }
            Error::InvalidTimestamp => {
                write!(f, "Invalid timestamp")
            }
        }
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(_err: chrono::format::ParseError) -> Error {
        Error::InvalidInputString
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_err: TryFromSliceError) -> Error {
        Error::InvalidInputString
    }
}
