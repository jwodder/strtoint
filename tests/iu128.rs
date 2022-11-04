use core::num::{NonZeroI128, NonZeroU128};
use strtoint::{strtoint, StrToIntError};
use test_case::test_case;

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[test_case("-170141183460469231731687303715884105728", -170141183460469231731687303715884105728)]
fn test_strtoint_i128(s: &str, x: i128) {
    assert_eq!(strtoint::<i128>(s).unwrap(), x);
}

#[test_case("170141183460469231731687303715884105728", StrToIntError::OutOfRange)]
#[test_case("-170141183460469231731687303715884105729", StrToIntError::OutOfRange)]
fn test_strtoint_i128_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<i128>(s).unwrap_err(), err);
}

#[test_case("0", 0)]
#[test_case("1", 1)]
#[test_case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[test_case(
    "170141183460469231731687303715884105728",
    170141183460469231731687303715884105728
)]
#[test_case(
    "340282366920938463463374607431768211455",
    340282366920938463463374607431768211455
)]
fn test_strtoint_u128(s: &str, x: u128) {
    assert_eq!(strtoint::<u128>(s).unwrap(), x);
}

#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-170141183460469231731687303715884105727", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-170141183460469231731687303715884105728", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("340282366920938463463374607431768211456", StrToIntError::OutOfRange)]
fn test_strtoint_u128_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<u128>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case("-1", -1; "neg1")]
#[test_case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[test_case("-170141183460469231731687303715884105728", -170141183460469231731687303715884105728)]
fn test_strtoint_nonzero_i128(s: &str, x: i128) {
    assert_eq!(
        strtoint::<NonZeroI128>(s).unwrap(),
        NonZeroI128::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("170141183460469231731687303715884105728", StrToIntError::OutOfRange)]
#[test_case("-170141183460469231731687303715884105729", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i128_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI128>(s).unwrap_err(), err);
}

#[test_case("1", 1)]
#[test_case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[test_case(
    "170141183460469231731687303715884105728",
    170141183460469231731687303715884105728
)]
#[test_case(
    "340282366920938463463374607431768211455",
    340282366920938463463374607431768211455
)]
fn test_strtoint_nonzero_u128(s: &str, x: u128) {
    assert_eq!(
        strtoint::<NonZeroU128>(s).unwrap(),
        NonZeroU128::new(x).unwrap()
    );
}

#[test_case("0", StrToIntError::OutOfRange)]
#[test_case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-170141183460469231731687303715884105727", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("-170141183460469231731687303715884105728", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[test_case("340282366920938463463374607431768211456", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u128_err(s: &str, err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU128>(s).unwrap_err(), err);
}
