mod error;
mod internal;

pub type Error = crate::error::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;
use chrono::prelude::*;

#[inline]
pub fn seconds_since_epoch(s: &str) -> Result<i64> {
    Ok(internal::datetime_since_epoch_bit_manipulation(s)?.timestamp())
}

#[inline]
pub fn nanos_since_epoch(s: &str) -> Result<i64> {
    Ok(internal::datetime_since_epoch_bit_manipulation(s)?.timestamp_nanos())
}

#[inline]
pub fn datetime(s: &str) -> Result<DateTime<Utc>> {
    internal::datetime_since_epoch_bit_manipulation(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_since_epoch_is_valid() {
        let result = seconds_since_epoch("2023-04-07T12:52:00.321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920, result.unwrap());
    }

    #[test]
    fn nanos_since_epoch_is_valid() {
        let result = nanos_since_epoch("2023-04-07T12:52:00.321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920321000000, result.unwrap());
    }
}
