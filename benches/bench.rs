#![feature(test)]
extern crate test;

use chrono::prelude::*;
use eeep::Result;
use test::{black_box, Bencher};

const TIMESTAMP: &str = "2023-04-07T15:10:20Z";
const EXPECTED: i64 = 1680880220000000000;

fn passthrough(_s: &str) -> Result<DateTime<Utc>> {
    let naive = match NaiveDateTime::from_timestamp_opt(1680880220, 0) {
        Some(naive) => naive,
        _ => return Err(eeep::Error::InvalidTimestamp),
    };
    Ok(DateTime::<Utc>::from_utc(naive, Utc))
}

fn baseline(s: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(s)?.into())
}

#[bench]
pub fn bench_baseline(b: &mut Bencher) {
    assert_eq!(EXPECTED, baseline(TIMESTAMP).unwrap().timestamp_nanos());
    b.bytes = TIMESTAMP.len() as u64;
    b.iter(|| baseline(black_box(TIMESTAMP)));
}

#[bench]
pub fn bench_passthrough(b: &mut Bencher) {
    assert_eq!(EXPECTED, passthrough(TIMESTAMP).unwrap().timestamp_nanos());
    b.bytes = TIMESTAMP.len() as u64;
    b.iter(|| passthrough(black_box(TIMESTAMP)));
}

#[bench]
pub fn datetime_since_epoch(b: &mut Bencher) {
    assert_eq!(
        EXPECTED,
        datetime_since_epoch_bit_manipulation(TIMESTAMP)
            .unwrap()
            .timestamp_nanos()
    );
    b.bytes = TIMESTAMP.len() as u64;
    b.iter(|| datetime_since_epoch_bit_manipulation(black_box(TIMESTAMP)));
}
