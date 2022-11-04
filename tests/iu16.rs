use rstest::rstest;
use strtoint::{strtoint, StrToIntError};

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case("32767", 32767)]
#[case("-32768", -32768)]
fn test_strtoint_i16(#[case] s: &str, #[case] x: i16) {
    assert_eq!(strtoint::<i16>(s).unwrap(), x);
}

#[rstest]
#[case("32768", StrToIntError::OutOfRange)]
#[case("-32769", StrToIntError::OutOfRange)]
fn test_strtoint_i16_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<i16>(s).unwrap_err(), err);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("32767", 32767)]
#[case("32768", 32768)]
#[case("65535", 65535)]
fn test_strtoint_u16(#[case] s: &str, #[case] x: u16) {
    assert_eq!(strtoint::<u16>(s).unwrap(), x);
}

#[rstest]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-32768", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-32769", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("65536", StrToIntError::OutOfRange)]
fn test_strtoint_u16_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<u16>(s).unwrap_err(), err);
}
