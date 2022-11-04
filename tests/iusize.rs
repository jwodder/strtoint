use rstest::rstest;
use strtoint::{strtoint, StrToIntError};

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case("32767", 32767)]
#[case("-32768", -32768)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    case("2147483647", 2147483647)
)]
#[cfg_attr(any(target_pointer_width = "32", target_pointer_width = "64"), case("-2147483648", -2147483648))]
#[cfg_attr(
    target_pointer_width = "64",
    case("9223372036854775807", 9223372036854775807)
)]
#[cfg_attr(target_pointer_width = "64", case("-9223372036854775808", -9223372036854775808))]
fn test_strtoint_isize(#[case] s: &str, #[case] x: isize) {
    assert_eq!(strtoint::<isize>(s).unwrap(), x);
}

#[rstest]
#[cfg_attr(target_pointer_width = "16", case("32768", StrToIntError::OutOfRange))]
#[cfg_attr(target_pointer_width = "16", case("-32769", StrToIntError::OutOfRange))]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    case("2147483648", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    case("-2147483649", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    case("9223372036854775808", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    case("-9223372036854775809", StrToIntError::OutOfRange)
)]
#[case(
    "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff",
    StrToIntError::OutOfRange
)]
fn test_strtoint_isize_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<isize>(s).unwrap_err(), err);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("32767", 32767)]
#[case("32768", 32768)]
#[case("65535", 65535)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    case("2147483647", 2147483647)
)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    case("2147483648", 2147483648)
)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    case("4294967295", 4294967295)
)]
#[cfg_attr(
    target_pointer_width = "64",
    case("9223372036854775807", 9223372036854775807)
)]
#[cfg_attr(
    target_pointer_width = "64",
    case("9223372036854775808", 9223372036854775808)
)]
#[cfg_attr(
    target_pointer_width = "64",
    case("18446744073709551615", 18446744073709551615)
)]
fn test_strtoint_usize(#[case] s: &str, #[case] x: usize) {
    assert_eq!(strtoint::<usize>(s).unwrap(), x);
}

#[rstest]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-32768", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-32769", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[cfg_attr(target_pointer_width = "16", case("65536", StrToIntError::OutOfRange))]
#[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), case("-2147483648", StrToIntError::InvalidCharacter {c: '-', position: 0}))]
#[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), case("-2147483649", StrToIntError::InvalidCharacter {c: '-', position: 0}))]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    case("4294967296", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"),
    case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"),
    case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    case("18446744073709551616", StrToIntError::OutOfRange)
)]
#[case(
    "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff",
    StrToIntError::OutOfRange
)]
fn test_strtoint_usize_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<usize>(s).unwrap_err(), err);
}
