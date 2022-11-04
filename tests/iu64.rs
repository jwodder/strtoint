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
