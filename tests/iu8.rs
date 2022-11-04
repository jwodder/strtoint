use core::num::{NonZeroI8, NonZeroU8};
use rstest::rstest;
use strtoint::{strtoint, StrToIntError};

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case("127", 127)]
#[case("-128", -128)]
fn test_strtoint_i8(#[case] s: &str, #[case] x: i8) {
    assert_eq!(strtoint::<i8>(s).unwrap(), x);
}

#[rstest]
#[case("128", StrToIntError::OutOfRange)]
#[case("-129", StrToIntError::OutOfRange)]
fn test_strtoint_i8_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<i8>(s).unwrap_err(), err);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("127", 127)]
#[case("128", 128)]
#[case("255", 255)]
fn test_strtoint_u8(#[case] s: &str, #[case] x: u8) {
    assert_eq!(strtoint::<u8>(s).unwrap(), x);
}

#[rstest]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-128", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-129", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("256", StrToIntError::OutOfRange)]
fn test_strtoint_u8_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<u8>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("-1", -1)]
#[case("127", 127)]
#[case("-128", -128)]
fn test_strtoint_nonzero_i8(#[case] s: &str, #[case] x: i8) {
    assert_eq!(
        strtoint::<NonZeroI8>(s).unwrap(),
        NonZeroI8::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("128", StrToIntError::OutOfRange)]
#[case("-129", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i8_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI8>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("127", 127)]
#[case("128", 128)]
#[case("255", 255)]
fn test_strtoint_nonzero_u8(#[case] s: &str, #[case] x: u8) {
    assert_eq!(
        strtoint::<NonZeroU8>(s).unwrap(),
        NonZeroU8::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-128", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-129", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("256", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u8_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU8>(s).unwrap_err(), err);
}
