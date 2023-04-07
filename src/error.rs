use std::array::TryFromSliceError;

#[derive(Debug)]
pub enum EeepError {
    InvalidInputString,
    InvalidTimestamp,
}

impl std::error::Error for EeepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            EeepError::InvalidInputString => None,
            EeepError::InvalidTimestamp => None,
        }
    }
}

impl std::fmt::Display for EeepError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EeepError::InvalidInputString => {
                write!(f, "Invalid input string")
            }
            EeepError::InvalidTimestamp => {
                write!(f, "Invalid timestamp")
            }
        }
    }
}

impl From<chrono::format::ParseError> for EeepError {
    fn from(_err: chrono::format::ParseError) -> EeepError {
        EeepError::InvalidInputString
    }
}

impl From<TryFromSliceError> for EeepError {
    fn from(_err: TryFromSliceError) -> EeepError {
        EeepError::InvalidInputString
    }
}
