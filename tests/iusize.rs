#![cfg(test)]
use core::num::{NonZeroIsize, NonZeroUsize};
use strtoint::{strtoint, StrToIntError};
use test_case::test_case;

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("32767", 32767)]
#[test_case("-32768", -32768)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("2147483647", 2147483647)
)]
#[cfg_attr(any(target_pointer_width = "32", target_pointer_width = "64"), test_case("-2147483648", -2147483648))]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("9223372036854775807", 9223372036854775807)
)]
#[cfg_attr(target_pointer_width = "64", test_case("-9223372036854775808", -9223372036854775808))]
fn test_strtoint_isize(s: &str, x: isize) {
    assert_eq!(strtoint::<isize>(s).unwrap(), x);
}

#[cfg_attr(
    target_pointer_width = "16",
    test_case("32768", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    target_pointer_width = "16",
    test_case("-32769", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    test_case("2147483648", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    test_case("-2147483649", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    test_case("9223372036854775808", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    test_case("-9223372036854775809", StrToIntError::OutOfRange)
)]
#[test_case(
    "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff",
    StrToIntError::OutOfRange
)]
fn test_strtoint_isize_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<isize>(s).unwrap_err(), err);
}

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("32767", 32767)]
#[test_case("32768", 32768)]
#[test_case("65535", 65535)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("2147483647", 2147483647)
)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("2147483648", 2147483648)
)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("4294967295", 4294967295)
)]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("9223372036854775807", 9223372036854775807)
)]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("9223372036854775808", 9223372036854775808)
)]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("18446744073709551615", 18446744073709551615)
)]
fn test_strtoint_usize(s: &str, x: usize) {
    assert_eq!(strtoint::<usize>(s).unwrap(), x);
}

#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32768", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32769", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[cfg_attr(
    target_pointer_width = "16",
    test_case("65536", StrToIntError::OutOfRange)
)]
#[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), test_case("-2147483648", StrToIntError::InvalidCharacter {c: '-', position: 0}))]
#[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), test_case("-2147483649", StrToIntError::InvalidCharacter {c: '-', position: 0}))]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    test_case("4294967296", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"),
    test_case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"),
    test_case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    test_case("18446744073709551616", StrToIntError::OutOfRange)
)]
#[test_case(
    "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff",
    StrToIntError::OutOfRange
)]
fn test_strtoint_usize_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<usize>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("32767", 32767)]
#[test_case("-32768", -32768)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("2147483647", 2147483647)
)]
#[cfg_attr(any(target_pointer_width = "32", target_pointer_width = "64"), test_case("-2147483648", -2147483648))]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("9223372036854775807", 9223372036854775807)
)]
#[cfg_attr(target_pointer_width = "64", test_case("-9223372036854775808", -9223372036854775808))]
fn test_strtoint_nonzero_isize(s: &str, x: isize) {
    assert_eq!(
        strtoint::<NonZeroIsize>(s).unwrap(),
        NonZeroIsize::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[cfg_attr(
    target_pointer_width = "16",
    test_case("32768", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    target_pointer_width = "16",
    test_case("-32769", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    test_case("2147483648", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    test_case("-2147483649", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    test_case("9223372036854775808", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    test_case("-9223372036854775809", StrToIntError::OutOfRange)
)]
#[test_case(
    "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff",
    StrToIntError::OutOfRange
)]
fn test_strtoint_nonzero_isize_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroIsize>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("32767", 32767)]
#[test_case("32768", 32768)]
#[test_case("65535", 65535)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("2147483647", 2147483647)
)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("2147483648", 2147483648)
)]
#[cfg_attr(
    any(target_pointer_width = "32", target_pointer_width = "64"),
    test_case("4294967295", 4294967295)
)]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("9223372036854775807", 9223372036854775807)
)]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("9223372036854775808", 9223372036854775808)
)]
#[cfg_attr(
    target_pointer_width = "64",
    test_case("18446744073709551615", 18446744073709551615)
)]
fn test_strtoint_nonzero_usize(s: &str, x: usize) {
    assert_eq!(
        strtoint::<NonZeroUsize>(s).unwrap(),
        NonZeroUsize::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32768", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32769", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[cfg_attr(
    target_pointer_width = "16",
    test_case("65536", StrToIntError::OutOfRange)
)]
#[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), test_case("-2147483648", StrToIntError::InvalidCharacter {c: '-', position: 0}))]
#[cfg_attr(any(target_pointer_width = "16", target_pointer_width = "32"), test_case("-2147483649", StrToIntError::InvalidCharacter {c: '-', position: 0}))]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32"),
    test_case("4294967296", StrToIntError::OutOfRange)
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"),
    test_case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})
)]
#[cfg_attr(
    any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"),
    test_case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})
)]
#[cfg_attr(
    any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
    ),
    test_case("18446744073709551616", StrToIntError::OutOfRange)
)]
#[test_case(
    "0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff",
    StrToIntError::OutOfRange
)]
fn test_strtoint_nonzero_usize_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroUsize>(s).unwrap_err(), err);
}
