use std::error::Error;

pub fn seconds_since_epoch(_s: &str) -> Result<i64, Box<dyn Error>> {
    Ok(0)
}

pub fn nanoseconds_since_epoch(_s: &str) -> Result<i64, Box<dyn Error>> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_since_epoch_is_valid() {
        let result = seconds_since_epoch(&"2023-04-07T12:52:00.321");
        assert!(result.is_ok());
        assert_eq!(1680871920, result.unwrap());
    }

    #[test]
    fn nanoseconds_since_epoch_is_valid() {
        let result = seconds_since_epoch(&"2023-04-07T12:52:00.321");
        assert!(result.is_ok());
        assert_eq!(1680871920321000000, result.unwrap());
    }
}
