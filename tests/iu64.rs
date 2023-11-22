#![cfg(test)]
use core::num::{NonZeroI64, NonZeroU64};
use strtoint::{strtoint, StrToIntError};
use test_case::test_case;

#[test_case("0", 0; "zero")]
#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("9223372036854775807", 9223372036854775807)]
#[test_case("-9223372036854775808", -9223372036854775808)]
fn test_strtoint_i64(s: &str, x: i64) {
    assert_eq!(strtoint::<i64>(s).unwrap(), x);
}

#[test_case("9223372036854775808", StrToIntError::OutOfRange)]
#[test_case("-9223372036854775809", StrToIntError::OutOfRange)]
fn test_strtoint_i64_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<i64>(s).unwrap_err(), err);
}

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("9223372036854775807", 9223372036854775807)]
#[test_case("9223372036854775808", 9223372036854775808)]
#[test_case("18446744073709551615", 18446744073709551615)]
fn test_strtoint_u64(s: &str, x: u64) {
    assert_eq!(strtoint::<u64>(s).unwrap(), x);
}

#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("18446744073709551616", StrToIntError::OutOfRange)]
fn test_strtoint_u64_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<u64>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("9223372036854775807", 9223372036854775807)]
#[test_case("-9223372036854775808", -9223372036854775808)]
fn test_strtoint_nonzero_i64(s: &str, x: i64) {
    assert_eq!(
        strtoint::<NonZeroI64>(s).unwrap(),
        NonZeroI64::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("9223372036854775808", StrToIntError::OutOfRange)]
#[test_case("-9223372036854775809", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i64_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI64>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("9223372036854775807", 9223372036854775807)]
#[test_case("9223372036854775808", 9223372036854775808)]
#[test_case("18446744073709551615", 18446744073709551615)]
fn test_strtoint_nonzero_u64(s: &str, x: u64) {
    assert_eq!(
        strtoint::<NonZeroU64>(s).unwrap(),
        NonZeroU64::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("18446744073709551616", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u64_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU64>(s).unwrap_err(), err);
}
