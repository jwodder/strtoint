use core::num::{NonZeroI64, NonZeroU64};
use rstest::rstest;
use strtoint::{strtoint, StrToIntError};

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case("9223372036854775807", 9223372036854775807)]
#[case("-9223372036854775808", -9223372036854775808)]
fn test_strtoint_i64(#[case] s: &str, #[case] x: i64) {
    assert_eq!(strtoint::<i64>(s).unwrap(), x);
}

#[rstest]
#[case("9223372036854775808", StrToIntError::OutOfRange)]
#[case("-9223372036854775809", StrToIntError::OutOfRange)]
fn test_strtoint_i64_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<i64>(s).unwrap_err(), err);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("9223372036854775807", 9223372036854775807)]
#[case("9223372036854775808", 9223372036854775808)]
#[case("18446744073709551615", 18446744073709551615)]
fn test_strtoint_u64(#[case] s: &str, #[case] x: u64) {
    assert_eq!(strtoint::<u64>(s).unwrap(), x);
}

#[rstest]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("18446744073709551616", StrToIntError::OutOfRange)]
fn test_strtoint_u64_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<u64>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("-1", -1)]
#[case("9223372036854775807", 9223372036854775807)]
#[case("-9223372036854775808", -9223372036854775808)]
fn test_strtoint_nonzero_i64(#[case] s: &str, #[case] x: i64) {
    assert_eq!(
        strtoint::<NonZeroI64>(s).unwrap(),
        NonZeroI64::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("9223372036854775808", StrToIntError::OutOfRange)]
#[case("-9223372036854775809", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i64_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI64>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("9223372036854775807", 9223372036854775807)]
#[case("9223372036854775808", 9223372036854775808)]
#[case("18446744073709551615", 18446744073709551615)]
fn test_strtoint_nonzero_u64(#[case] s: &str, #[case] x: u64) {
    assert_eq!(
        strtoint::<NonZeroU64>(s).unwrap(),
        NonZeroU64::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("18446744073709551616", StrToIntError::OutOfRange)]
fn test_strtoint_nonzeri_u64_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU64>(s).unwrap_err(), err);
}
