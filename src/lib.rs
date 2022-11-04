use core::fmt;

pub trait StrToInt {
    type Err;

    fn strtoint(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

pub fn strtoint<T: StrToInt>(s: &str) -> Result<T, <T as StrToInt>::Err> {
    T::strtoint(s)
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum StrToIntError {
    NoDigits,
    InvalidCharacter { c: char, position: usize },
    OutOfRange,
}

impl fmt::Display for StrToIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrToIntError::NoDigits => write!(f, "no digits in input"),
            StrToIntError::InvalidCharacter { c, position } => {
                write!(f, "invalid character {:?} at position {}", c, position)
            }
            StrToIntError::OutOfRange => write!(f, "value is out of range for numeric type"),
        }
    }
}

// TODO: Unstable?
//impl core::error::Error for StrToIntError {}

macro_rules! implement {
    ($($t:ty),* $(,)?) => {
      $(
        impl StrToInt for $t {
            type Err = StrToIntError;

            fn strtoint(mut s: &str) -> Result<Self, Self::Err>
            where
                Self: Sized,
            {
                let mut offset = 0;
                let is_positive = {
                    if let Some(t) = s.strip_prefix('+') {
                        offset += 1;
                        s = t;
                        true
                    } else if let Some(t) = s.strip_prefix('-') {
                        if <$t>::MIN == 0 {
                            return Err(StrToIntError::InvalidCharacter {c: '-', position: 0});
                        }
                        offset += 1;
                        s = t;
                        false
                    } else {
                        true
                    }
                };
                let radix = {
                    if let Some(t) = s.strip_prefix("0x") {
                        offset += 2;
                        s = t;
                        16
                    } else if let Some(t) = s.strip_prefix("0o") {
                        offset += 2;
                        s = t;
                        8
                    } else if let Some(t) = s.strip_prefix("0b") {
                        offset += 2;
                        s = t;
                        2
                    } else {
                        10
                    }
                };
                let mut value: $t = 0;
                let mut digit_seen = false;
                for (i, c) in s.char_indices() {
                    if c == '_' {
                        if !digit_seen && radix == 10 {
                            return Err(StrToIntError::InvalidCharacter{c, position: i + offset});
                        }
                        continue;
                    }
                    let digit = c
                        .to_digit(radix)
                        .ok_or_else(|| StrToIntError::InvalidCharacter {
                            c,
                            position: i + offset,
                        })?;
                    value = value
                        .checked_mul(radix as $t)
                        .ok_or(StrToIntError::OutOfRange)?;
                    value = if is_positive {
                        value.checked_add(digit as $t)
                    } else {
                        value.checked_sub(digit as $t)
                    }.ok_or(StrToIntError::OutOfRange)?;
                    digit_seen = true;
                }
                if !digit_seen {
                    return Err(StrToIntError::NoDigits);
                }
                Ok(value)
            }
        }
      )*
    }
}

implement!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
    #[case("0x-", StrToIntError::InvalidCharacter {c: '-', position: 2})]
    #[case("0o-", StrToIntError::InvalidCharacter {c: '-', position: 2})]
    #[case("0b-", StrToIntError::InvalidCharacter {c: '-', position: 2})]
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

    #[rstest]
    #[case("0", 0)]
    #[case("1", 1)]
    #[case("-1", -1)]
    #[case("9223372036854775807", 9223372036854775807)]
    #[case("-9223372036854775808", -9223372036854775808)]
    fn test_strtoint_i64(#[case] s: &str, #[case] x: i64) {
        assert_eq!(strtoint::<i64>(s).unwrap(), x);
    }

    #[rstest]
    #[case("9223372036854775808", StrToIntError::OutOfRange)]
    #[case("-9223372036854775809", StrToIntError::OutOfRange)]
    fn test_strtoint_i64_err(#[case] s: &str, #[case] err: StrToIntError) {
        assert_eq!(strtoint::<i64>(s).unwrap_err(), err);
    }

    #[rstest]
    #[case("0", 0)]
    #[case("1", 1)]
    #[case("9223372036854775807", 9223372036854775807)]
    #[case("9223372036854775808", 9223372036854775808)]
    #[case("18446744073709551615", 18446744073709551615)]
    fn test_strtoint_u64(#[case] s: &str, #[case] x: u64) {
        assert_eq!(strtoint::<u64>(s).unwrap(), x);
    }

    #[rstest]
    #[case("-1", StrToIntError::InvalidCharacter {c: '-', position: 0})]
    #[case("-9223372036854775807", StrToIntError::InvalidCharacter {c: '-', position: 0})]
    #[case("-9223372036854775808", StrToIntError::InvalidCharacter {c: '-', position: 0})]
    #[case("18446744073709551616", StrToIntError::OutOfRange)]
    fn test_strtoint_u64_err(#[case] s: &str, #[case] err: StrToIntError) {
        assert_eq!(strtoint::<u64>(s).unwrap_err(), err);
    }

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

    #[test]
    fn test_display_error_no_digits() {
        assert_eq!(StrToIntError::NoDigits.to_string(), "no digits in input");
    }

    #[test]
    fn test_display_error_invalid_character() {
        assert_eq!(
            StrToIntError::InvalidCharacter {
                c: '.',
                position: 2
            }
            .to_string(),
            "invalid character '.' at position 2"
        );
    }

    #[test]
    fn test_display_error_out_of_range() {
        assert_eq!(
            StrToIntError::OutOfRange.to_string(),
            "value is out of range for numeric type"
        );
    }
}
