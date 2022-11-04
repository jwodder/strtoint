use core::num::{NonZeroI128, NonZeroU128};
use rstest::rstest;
use strtoint::{strtoint, StrToIntError};

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case("-1", -1)]
#[case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[case("-170141183460469231731687303715884105728", -170141183460469231731687303715884105728)]
fn test_strtoint_i128(#[case] s: &str, #[case] x: i128) {
    assert_eq!(strtoint::<i128>(s).unwrap(), x);
}

#[rstest]
#[case("170141183460469231731687303715884105728", StrToIntError::OutOfRange)]
#[case("-170141183460469231731687303715884105729", StrToIntError::OutOfRange)]
fn test_strtoint_i128_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<i128>(s).unwrap_err(), err);
}

#[rstest]
#[case("0", 0)]
#[case("1", 1)]
#[case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[case(
    "170141183460469231731687303715884105728",
    170141183460469231731687303715884105728
)]
#[case(
    "340282366920938463463374607431768211455",
    340282366920938463463374607431768211455
)]
fn test_strtoint_u128(#[case] s: &str, #[case] x: u128) {
    assert_eq!(strtoint::<u128>(s).unwrap(), x);
}

#[rstest]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-170141183460469231731687303715884105727", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-170141183460469231731687303715884105728", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("340282366920938463463374607431768211456", StrToIntError::OutOfRange)]
fn test_strtoint_u128_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<u128>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case("-1", -1)]
#[case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[case("-170141183460469231731687303715884105728", -170141183460469231731687303715884105728)]
fn test_strtoint_nonzero_i128(#[case] s: &str, #[case] x: i128) {
    assert_eq!(
        strtoint::<NonZeroI128>(s).unwrap(),
        NonZeroI128::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("170141183460469231731687303715884105728", StrToIntError::OutOfRange)]
#[case("-170141183460469231731687303715884105729", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_i128_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroI128>(s).unwrap_err(), err);
}

#[rstest]
#[case("1", 1)]
#[case(
    "170141183460469231731687303715884105727",
    170141183460469231731687303715884105727
)]
#[case(
    "170141183460469231731687303715884105728",
    170141183460469231731687303715884105728
)]
#[case(
    "340282366920938463463374607431768211455",
    340282366920938463463374607431768211455
)]
fn test_strtoint_nonzero_u128(#[case] s: &str, #[case] x: u128) {
    assert_eq!(
        strtoint::<NonZeroU128>(s).unwrap(),
        NonZeroU128::new(x).unwrap()
    );
}

#[rstest]
#[case("0", StrToIntError::OutOfRange)]
#[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-170141183460469231731687303715884105727", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("-170141183460469231731687303715884105728", StrToIntError::InvalidCharacter {c: '-', position: 0})]
#[case("340282366920938463463374607431768211456", StrToIntError::OutOfRange)]
fn test_strtoint_nonzero_u128_err(#[case] s: &str, #[case] err: StrToIntError) {
    assert_eq!(strtoint::<NonZeroU128>(s).unwrap_err(), err);
}
