#![cfg(test)]
use core::num::{NonZeroI32, NonZeroU32};
use strtoint::{strtoint, StrToIntError};
use test_case::test_case;

#[test_case("0", 0)]
#[test_case("+0", 0; "pos0")]
#[test_case("-0", 0; "neg0")]
#[test_case("1", 1)]
#[test_case("+1", 1; "pos1")]
#[test_case("-1", -1; "neg1")]
#[test_case("10", 10)]
#[test_case("0x10", 16)]
#[test_case("0o10", 8)]
#[test_case("0b10", 2)]
#[test_case("+10", 10; "pos10")]
#[test_case("+0x10", 16; "pos_hex_10")]
#[test_case("+0o10", 8; "pos_oct_10")]
#[test_case("+0b10", 2; "pos_bin_10")]
#[test_case("-10", -10; "neg10")]
#[test_case("-0x10", -16; "neg_hex_10")]
#[test_case("-0o10", -8; "neg_oct_10")]
#[test_case("-0b10", -2; "neg_bin_10")]
#[test_case("123_456", 123_456)]
#[test_case("-2147483648", -2147483648)]
#[test_case("-0x80000000", -2147483648)]
#[test_case("-0o20000000000", -2147483648)]
#[test_case("-0b10000000000000000000000000000000", -2147483648)]
#[test_case("2147483647", 2147483647)]
#[test_case("0x7fFFffFF", 2147483647)]
#[test_case("0o17777777777", 2147483647)]
#[test_case("0b1111111111111111111111111111111", 2147483647)]
#[test_case("0x___1___", 1; "hex_gap_1")]
#[test_case("0o___1___", 1; "oct_gap_1")]
#[test_case("0b___1___", 1; "bin_gap_1")]
#[test_case("1___", 1; "1gap")]
#[test_case("0___", 0; "0gap")]
#[test_case("0644", 644)]
#[test_case("00000000000000000000000000000000000000001", 1)]
#[test_case("0_______________________________________1", 1)]
fn test_strtoint_i32(s: &str, x: i32) {
    assert_eq!(strtoint::<i32>(s).unwrap(), x);
}

#[test_case("", StrToIntError::NoDigits; "empty")]
#[test_case("+", StrToIntError::NoDigits; "plus")]
#[test_case("-", StrToIntError::NoDigits; "minus")]
#[test_case("_", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[test_case("0x", StrToIntError::NoDigits)]
#[test_case("0o", StrToIntError::NoDigits)]
#[test_case("0b", StrToIntError::NoDigits)]
#[test_case("0x+", StrToIntError::InvalidCharacter {c: '+', position: 2}; "hex_plus")]
#[test_case("0o+", StrToIntError::InvalidCharacter {c: '+', position: 2}; "oct_plus")]
#[test_case("0b+", StrToIntError::InvalidCharacter {c: '+', position: 2}; "bin_plus")]
#[test_case("0x+123", StrToIntError::InvalidCharacter {c: '+', position: 2}; "hex_pos_123")]
#[test_case("0x-", StrToIntError::InvalidCharacter {c: '-', position: 2}; "hex_minus")]
#[test_case("0o-", StrToIntError::InvalidCharacter {c: '-', position: 2}; "oct_minus")]
#[test_case("0b-", StrToIntError::InvalidCharacter {c: '-', position: 2}; "bin_minus")]
#[test_case("0x-123", StrToIntError::InvalidCharacter {c: '-', position: 2}; "hex_neg_123")]
#[test_case("0x_", StrToIntError::NoDigits; "hex_under")]
#[test_case("0o_", StrToIntError::NoDigits; "oct_under")]
#[test_case("0b_", StrToIntError::NoDigits; "bin_under")]
#[test_case("0xg", StrToIntError::InvalidCharacter {c: 'g', position: 2})]
#[test_case("0o9", StrToIntError::InvalidCharacter {c: '9', position: 2})]
#[test_case("0b2", StrToIntError::InvalidCharacter {c: '2', position: 2})]
#[test_case("feed", StrToIntError::InvalidCharacter {c: 'f', position: 0})]
#[test_case(" 42 ", StrToIntError::InvalidCharacter {c: ' ', position: 0}; "42space")]
#[test_case("42.", StrToIntError::InvalidCharacter {c: '.', position: 2}; "42dot")]
#[test_case("42.0", StrToIntError::InvalidCharacter {c: '.', position: 2})]
#[test_case("<=>", StrToIntError::InvalidCharacter {c: '<', position: 0}; "cmp")]
#[test_case("2147483648", StrToIntError::OutOfRange)]
#[test_case("0x80000000", StrToIntError::OutOfRange)]
#[test_case("0o20000000000", StrToIntError::OutOfRange)]
#[test_case("0b10000000000000000000000000000000", StrToIntError::OutOfRange)]
#[test_case("-2147483649", StrToIntError::OutOfRange)]
#[test_case("123456789012345678902134567890", StrToIntError::OutOfRange; "very_big")]
#[test_case("-123456789012345678902134567890", StrToIntError::OutOfRange; "neg_very_big")]
#[test_case("0X10", StrToIntError::InvalidCharacter {c: 'X', position: 1})]
#[test_case("0O10", StrToIntError::InvalidCharacter {c: 'O', position: 1})]
#[test_case("0B10", StrToIntError::InvalidCharacter {c: 'B', position: 1})]
#[test_case("+0X10", StrToIntError::InvalidCharacter {c: 'X', position: 2}; "pos_upper_0X")]
#[test_case("+0O10", StrToIntError::InvalidCharacter {c: 'O', position: 2}; "pos_upper_0O")]
#[test_case("+0B10", StrToIntError::InvalidCharacter {c: 'B', position: 2}; "pos_upper_0B")]
#[test_case("-0X10", StrToIntError::InvalidCharacter {c: 'X', position: 2}; "neg_upper_0X")]
#[test_case("-0O10", StrToIntError::InvalidCharacter {c: 'O', position: 2}; "neg_upper_0O")]
#[test_case("-0B10", StrToIntError::InvalidCharacter {c: 'B', position: 2}; "neg_upper_0B")]
#[test_case("___1___", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[test_case("_0x10", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[test_case("_0o10", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[test_case("_0b10", StrToIntError::InvalidCharacter {c: '_', position: 0})]
#[test_case("12³45", StrToIntError::InvalidCharacter {c: '³', position: 2}; "super3")]
fn test_strtoint_i32_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<i32>(s).unwrap_err(), err);
}

#[test_case("0", 0)]
#[test_case("+0", 0; "pos0")]
#[test_case("1", 1)]
#[test_case("2147483647", 2147483647)]
#[test_case("2147483648", 2147483648)]
#[test_case("4294967295", 4294967295)]
fn test_strtoint_u32(s: &str, x: u32) {
    assert_eq!(strtoint::<u32>(s).unwrap(), x);
}

#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-0", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("4294967296", StrToIntError::OutOfRange)]
fn test_strtoint_u32_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<u32>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("2147483647", 2147483647)]
#[test_case("-2147483648", -2147483648)]
fn test_strtoint_nonzero_i32(s: &str, x: i32) {
    assert_eq!(
        strtoint::<NonZeroI32>(s).unwrap(),
        NonZeroI32::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("2147483648", StrToIntError::OutOfRange)]
#[test_case("-2147483649", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i32_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI32>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("2147483647", 2147483647)]
#[test_case("2147483648", 2147483648)]
#[test_case("4294967295", 4294967295)]
fn test_strtoint_nonzero_u32(s: &str, x: u32) {
    assert_eq!(
        strtoint::<NonZeroU32>(s).unwrap(),
        NonZeroU32::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("4294967296", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u32_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU32>(s).unwrap_err(), err);
}
