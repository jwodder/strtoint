use core::num::{NonZeroI8, NonZeroU8};
use strtoint::{strtoint, StrToIntError};
use test_case::test_case;

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("127", 127)]
#[test_case("-128", -128)]
fn test_strtoint_i8(s: &str, x: i8) {
    assert_eq!(strtoint::<i8>(s).unwrap(), x);
}

#[test_case("128", StrToIntError::OutOfRange)]
#[test_case("-129", StrToIntError::OutOfRange)]
fn test_strtoint_i8_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<i8>(s).unwrap_err(), err);
}

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("127", 127)]
#[test_case("128", 128)]
#[test_case("255", 255)]
fn test_strtoint_u8(s: &str, x: u8) {
    assert_eq!(strtoint::<u8>(s).unwrap(), x);
}

#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-128", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-129", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("256", StrToIntError::OutOfRange)]
fn test_strtoint_u8_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<u8>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case("127", 127)]
#[test_case("-128", -128)]
fn test_strtoint_nonzero_i8(s: &str, x: i8) {
    assert_eq!(
        strtoint::<NonZeroI8>(s).unwrap(),
        NonZeroI8::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("128", StrToIntError::OutOfRange)]
#[test_case("-129", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i8_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI8>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("127", 127)]
#[test_case("128", 128)]
#[test_case("255", 255)]
fn test_strtoint_nonzero_u8(s: &str, x: u8) {
    assert_eq!(
        strtoint::<NonZeroU8>(s).unwrap(),
        NonZeroU8::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-128", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-129", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("256", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u8_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU8>(s).unwrap_err(), err);
}
