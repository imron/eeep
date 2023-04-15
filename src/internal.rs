use crate::Result;
use chrono::prelude::*;

#[inline]
fn datetime_with(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    nanos: u32,
) -> Result<DateTime<Utc>> {
    let naive = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => match date.and_hms_nano_opt(hour, minute, second, nanos) {
            Some(dt) => dt,
            None => Err(crate::Error::InvalidInputString)?,
        },
        None => Err(crate::Error::InvalidInputString)?,
    };

    Ok(DateTime::<Utc>::from_utc(naive, Utc))
}

pub fn datetime_since_epoch_bit_manipulation(s: &str) -> Result<DateTime<Utc>> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len < 20 {
        return Err(crate::Error::InvalidInputString);
    }

    let year = convert_4_digits(&bytes[..4])? as i32;

    if bytes[4] != b'-' {
        return Err(crate::Error::InvalidInputString);
    }

    let month = convert_2_digits(&bytes[5..7])?;

    if bytes[7] != b'-' {
        return Err(crate::Error::InvalidInputString);
    }

    let day = convert_2_digits(&bytes[8..10])?;

    if bytes[10] != b'T' {
        return Err(crate::Error::InvalidInputString);
    }

    let hour = convert_2_digits(&bytes[11..13])?;

    if bytes[13] != b':' {
        return Err(crate::Error::InvalidInputString);
    }

    let minute = convert_2_digits(&bytes[14..16])?;

    if bytes[16] != b':' {
        return Err(crate::Error::InvalidInputString);
    }

    let second = convert_2_digits(&bytes[17..19])?;

    let mut fractions = 0;
    let mut count = 0;
    let end = std::cmp::min(len, 30);
    for b in &bytes[20..end] {
        if !b.is_ascii_digit() {
            break;
        }
        fractions = (fractions * 10) + ((b - b'0') as u32);
        count += 1;
    }

    if count > 0 && bytes[19] != b'.' {
        return Err(crate::Error::InvalidInputString);
    }

    let nanos = match count {
        0 => fractions,
        1 => fractions * 100000000,
        2 => fractions * 10000000,
        3 => fractions * 1000000,
        4 => fractions * 100000,
        5 => fractions * 10000,
        6 => fractions * 1000,
        7 => fractions * 100,
        8 => fractions * 10,
        _ => fractions,
    };

    if end > len || bytes[end - 1] != b'Z' {
        return Err(crate::Error::InvalidInputString);
    }

    datetime_with(year, month, day, hour, minute, second, nanos)
}

#[inline]
fn convert_2_digits(digits: &[u8]) -> Result<u32> {
    let bytes: [u8; 2] = digits.try_into()?;
    let chunk = u16::from_ne_bytes(bytes) as u32;
    let lower = (chunk & 0x0f00) >> 8;
    let upper = (chunk & 0x000f) * 10;
    Ok(lower + upper)
}

#[inline]
fn convert_4_digits(digits: &[u8]) -> Result<u32> {
    let bytes: [u8; 4] = digits.try_into()?;
    let mut chunk = u32::from_ne_bytes(bytes) as u32;

    let mut lower = (chunk & 0x0f000f00) >> 8;
    let mut upper = (chunk & 0x000f000f) * 10;

    chunk = lower + upper;

    lower = (chunk & 0x00ff0000) >> 16;
    upper = (chunk & 0x000000ff) * 100;

    Ok(lower + upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_2_digits() {
        for i in 0..99 {
            let s = format!("{:2}", i);
            let actual = convert_2_digits(&s.as_bytes()[..2]).unwrap();
            assert_eq!(actual, i);
        }
    }

    #[test]
    fn test_convert_4_digits() {
        for i in 0..9999 {
            let s = format!("{:4}", i);
            let actual = convert_4_digits(&s.as_bytes()[..4]).unwrap();
            assert_eq!(i, actual);
        }
    }
}
