use core::num::{NonZeroI16, NonZeroU16};
use strtoint::{strtoint, StrToIntError};
use test_case::test_case;

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("32767", 32767)]
#[test_case("-32768", -32768)]
fn test_strtoint_i16(s: &str, x: i16) {
    assert_eq!(strtoint::<i16>(s).unwrap(), x);
}

#[test_case("32768", StrToIntError::OutOfRange)]
#[test_case("-32769", StrToIntError::OutOfRange)]
fn test_strtoint_i16_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<i16>(s).unwrap_err(), err);
}

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("32767", 32767)]
#[test_case("32768", 32768)]
#[test_case("65535", 65535)]
fn test_strtoint_u16(s: &str, x: u16) {
    assert_eq!(strtoint::<u16>(s).unwrap(), x);
}

#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32768", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32769", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("65536", StrToIntError::OutOfRange)]
fn test_strtoint_u16_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<u16>(s).unwrap_err(), err);
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

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("32768", StrToIntError::OutOfRange)]
#[test_case("-32769", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i16_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI16>(s).unwrap_err(), err);
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

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32768", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-32769", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("65536", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u16_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU16>(s).unwrap_err(), err);
}
