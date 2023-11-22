mod util;
use crate::util::assert_out_of_range;
use core::num::{NonZeroI16, NonZeroU16};
use strtoint::{strtoint, StrToPrimIntError};
use test_case::test_case;

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("32767", 32767)]
#[test_case("-32768", -32768)]
fn test_strtoint_i16(s: &str, x: i16) {
    assert_eq!(strtoint::<i16>(s).unwrap(), x);
}

#[test_case("32768")]
#[test_case("-32769")]
fn test_strtoint_i16_out_of_range(s: &str) {
    assert_out_of_range!(strtoint::<i16>(s).unwrap_err());
}

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("32767", 32767)]
#[test_case("32768", 32768)]
#[test_case("65535", 65535)]
fn test_strtoint_u16(s: &str, x: u16) {
    assert_eq!(strtoint::<u16>(s).unwrap(), x);
}

#[test_case("-1")]
#[test_case("-32768")]
#[test_case("-32769")]
fn test_strtoint_u16_negative(s: &str) {
    assert_eq!(strtoint::<u16>(s).unwrap_err(), StrToPrimIntError::InvalidCharacter {c: '-', position: 0});
}

#[test]
fn test_strtoint_u16_out_of_range() {
    let s = "65535";
    assert_out_of_range!(strtoint::<u16>(s).unwrap_err());
}

#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("32767", 32767)]
#[test_case("-32768", -32768)]
fn test_strtoint_nonzero_i16(s: &str, x: i16) {
    assert_eq!(
        strtoint::<NonZeroI16>(s).unwrap(),
        NonZeroI16::new(x).unwrap()
    );
}

#[test_case("0")]
#[test_case("32768")]
#[test_case("-32769")]
fn test_strtoint_nonzero_i16_out_of_range(s: &str) {
    assert_out_of_range!(strtoint::<NonZeroI16>(s).unwrap_err());
}

#[test_case("1", 1)]
#[test_case("32767", 32767)]
#[test_case("32768", 32768)]
#[test_case("65535", 65535)]
fn test_strtoint_nonzero_u16(s: &str, x: u16) {
    assert_eq!(
        strtoint::<NonZeroU16>(s).unwrap(),
        NonZeroU16::new(x).unwrap()
    );
}

#[test_case("-1")]
#[test_case("-32768")]
#[test_case("-32769")]
fn test_strtoint_nonzero_u16_negative(s: &str) {
    assert_eq!(strtoint::<NonZeroU16>(s).unwrap_err(), StrToPrimIntError::InvalidCharacter {c: '-', position: 0});
}

#[test_case("0")]
#[test_case("65536")]
fn test_strtoint_nonzero_u16_out_of_range(s: &str) {
    assert_out_of_range!(strtoint::<NonZeroU16>(s).unwrap_err());
}
