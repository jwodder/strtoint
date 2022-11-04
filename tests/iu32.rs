use core::num::{NonZeroI32, NonZeroU32};
use rstest::rstest;
use strtoint::{strtoint, StrToIntError};

#[rstest]
#[case("0", 0)]
#[case("+0", 0)]
#[case("-0", 0)]
#[case("1", 1)]
#[case("+1", 1)]
#[case("-1", -1)]
#[case("10", 10)]
#[case("0x10", 16)]
#[case("0o10", 8)]
#[case("0b10", 2)]
#[case("+10", 10)]
#[case("+0x10", 16)]
#[case("+0o10", 8)]
#[case("+0b10", 2)]
#[case("-10", -10)]
#[case("-0x10", -16)]
#[case("-0o10", -8)]
#[case("-0b10", -2)]
#[case("123_456", 123_456)]
#[case("-2147483648", -2147483648)]
#[case("-0x80000000", -2147483648)]
#[case("-0o20000000000", -2147483648)]
#[case("-0b10000000000000000000000000000000", -2147483648)]
#[case("2147483647", 2147483647)]
#[case("0x7fFFffFF", 2147483647)]
#[case("0o17777777777", 2147483647)]
#[case("0b1111111111111111111111111111111", 2147483647)]
#[case("0x___1___", 1)]
#[case("0o___1___", 1)]
#[case("0b___1___", 1)]
#[case("1___", 1)]
#[case("0___", 0)]
#[case("0644", 644)]
#[case("00000000000000000000000000000000000000001", 1)]
#[case("0_______________________________________1", 1)]
fn test_strtoint_i32(#[case] s: &str, #[case] x: i32) {
    assert_eq!(strtoint::<i32>(s).unwrap(), x);
}

#[rstest]
#[case("", StrToIntError::NoDigits)]
#[case("+", StrToIntError::NoDigits)]
#[case("-", StrToIntError::NoDigits)]
#[case("_", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[case("0x", StrToIntError::NoDigits)]
#[case("0o", StrToIntError::NoDigits)]
#[case("0b", StrToIntError::NoDigits)]
#[case("0x+", StrToIntError::InvalidCharacter {c: '+', position: 2})]
#[case("0o+", StrToIntError::InvalidCharacter {c: '+', position: 2})]
#[case("0b+", StrToIntError::InvalidCharacter {c: '+', position: 2})]
#[case("0x+123", StrToIntError::InvalidCharacter {c: '+', position: 2})]
#[case("0x-", StrToIntError::InvalidCharacter {c: '-', position: 2})]
#[case("0o-", StrToIntError::InvalidCharacter {c: '-', position: 2})]
#[case("0b-", StrToIntError::InvalidCharacter {c: '-', position: 2})]
#[case("0x-123", StrToIntError::InvalidCharacter {c: '-', position: 2})]
#[case("0x_", StrToIntError::NoDigits)]
#[case("0o_", StrToIntError::NoDigits)]
#[case("0b_", StrToIntError::NoDigits)]
#[case("0xg", StrToIntError::InvalidCharacter {c: 'g', position: 2})]
#[case("0o9", StrToIntError::InvalidCharacter {c: '9', position: 2})]
#[case("0b2", StrToIntError::InvalidCharacter {c: '2', position: 2})]
#[case("feed", StrToIntError::InvalidCharacter {c: 'f', position: 0})]
#[case(" 42 ", StrToIntError::InvalidCharacter {c: ' ', position: 0})]
#[case("42.", StrToIntError::InvalidCharacter {c: '.', position: 2})]
#[case("42.0", StrToIntError::InvalidCharacter {c: '.', position: 2})]
#[case("<=>", StrToIntError::InvalidCharacter {c: '<', position: 0})]
#[case("2147483648", StrToIntError::OutOfRange)]
#[case("0x80000000", StrToIntError::OutOfRange)]
#[case("0o20000000000", StrToIntError::OutOfRange)]
#[case("0b10000000000000000000000000000000", StrToIntError::OutOfRange)]
#[case("-2147483649", StrToIntError::OutOfRange)]
#[case("123456789012345678902134567890", StrToIntError::OutOfRange)]
#[case("-123456789012345678902134567890", StrToIntError::OutOfRange)]
#[case("0X10", StrToIntError::InvalidCharacter {c: 'X', position: 1})]
#[case("0O10", StrToIntError::InvalidCharacter {c: 'O', position: 1})]
#[case("0B10", StrToIntError::InvalidCharacter {c: 'B', position: 1})]
#[case("+0X10", StrToIntError::InvalidCharacter {c: 'X', position: 2})]
#[case("+0O10", StrToIntError::InvalidCharacter {c: 'O', position: 2})]
#[case("+0B10", StrToIntError::InvalidCharacter {c: 'B', position: 2})]
#[case("-0X10", StrToIntError::InvalidCharacter {c: 'X', position: 2})]
#[case("-0O10", StrToIntError::InvalidCharacter {c: 'O', position: 2})]
#[case("-0B10", StrToIntError::InvalidCharacter {c: 'B', position: 2})]
#[case("___1___", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[case("_0x10", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[case("_0o10", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[case("_0b10", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[case("12³45", StrToIntError::InvalidCharacter {c: '³', position: 2})]
fn test_strtoint_i32_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<i32>(s).unwrap_err(), err);
}

#[rstest]
#[case("0", 0)]
#[case("+0", 0)]
#[case("1", 1)]
#[case("2147483647", 2147483647)]
#[case("2147483648", 2147483648)]
#[case("4294967295", 4294967295)]
fn test_strtoint_u32(#[case] s: &str, #[case] x: u32) {
    assert_eq!(strtoint::<u32>(s).unwrap(), x);
}

#[rstest]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-0", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("4294967296", StrToIntError::OutOfRange)]
fn test_strtoint_u32_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<u32>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("-1", -1)]
#[case("2147483647", 2147483647)]
#[case("-2147483648", -2147483648)]
fn test_strtoint_nonzero_i32(#[case] s: &str, #[case] x: i32) {
    assert_eq!(
        strtoint::<NonZeroI32>(s).unwrap(),
        NonZeroI32::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("2147483648", StrToIntError::OutOfRange)]
#[case("-2147483649", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i32_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI32>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("2147483647", 2147483647)]
#[case("2147483648", 2147483648)]
#[case("4294967295", 4294967295)]
fn test_strtoint_nonzero_u32(#[case] s: &str, #[case] x: u32) {
    assert_eq!(
        strtoint::<NonZeroU32>(s).unwrap(),
        NonZeroU32::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("4294967296", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u32_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU32>(s).unwrap_err(), err);
}
