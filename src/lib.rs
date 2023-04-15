mod error;
mod internal;

pub type Error = crate::error::Error;
pub type Result<T> = std::result::Result<T, crate::Error>;
use chrono::prelude::*;

#[inline]
pub fn parse_from_timestamp(s: &str) -> Result<i64> {
    Ok(internal::datetime_since_epoch_bit_manipulation(s)?.timestamp())
}

#[inline]
pub fn parse_from_timestamp_nanos(s: &str) -> Result<i64> {
    Ok(internal::datetime_since_epoch_bit_manipulation(s)?.timestamp_nanos())
}

#[inline]
pub fn parse_from_timestamp_datetime(s: &str) -> Result<DateTime<Utc>> {
    internal::datetime_since_epoch_bit_manipulation(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_since_epoch_is_valid() {
        let result = parse_from_timestamp("2023-04-07T12:52:00.321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920, result.unwrap());
    }

    #[test]
    fn nanos_since_epoch_is_valid() {
        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.1Z");
        assert!(result.is_ok());
        assert_eq!(1680871920100000000, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.21Z");
        assert!(result.is_ok());
        assert_eq!(1680871920210000000, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920321000000, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.4321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920432100000, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.54321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920543210000, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.654321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920654321000, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.7654321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920765432100, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.87654321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920876543210, result.unwrap());

        let result = parse_from_timestamp_nanos("2023-04-07T12:52:00.987654321Z");
        assert!(result.is_ok());
        assert_eq!(1680871920987654321, result.unwrap());
    }
}
