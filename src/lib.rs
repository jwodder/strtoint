use core::fmt;

pub trait ParseInt {
    type Err;

    fn parse_int(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

pub fn parse_int<T: ParseInt>(s: &str) -> Result<T, <T as ParseInt>::Err> {
    T::parse_int(s)
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParseIntError {
    NoDigits,
    InvalidCharacter { c: char, position: usize },
    OutOfRange,
}

impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseIntError::NoDigits => write!(f, "no digits in input"),
            ParseIntError::InvalidCharacter { c, position } => {
                write!(f, "invalid character {:?} at byte position {}", c, position)
            }
            ParseIntError::OutOfRange => write!(f, "value is out of range for numeric type"),
        }
    }
}

// TODO: Unstable?
//impl core::error::Error for ParseIntError {}

macro_rules! implement {
    ($($t:ty),* $(,)?) => {
      $(
        impl ParseInt for $t {
            type Err = ParseIntError;

            fn parse_int(mut s: &str) -> Result<Self, Self::Err>
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
                            return Err(ParseIntError::InvalidCharacter {c: '-', position: 0});
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
                            return Err(ParseIntError::InvalidCharacter{c, position: i + offset});
                        }
                        continue;
                    }
                    let digit = c
                        .to_digit(radix)
                        .ok_or_else(|| ParseIntError::InvalidCharacter {
                            c,
                            position: i + offset,
                        })?;
                    value = value
                        .checked_mul(radix as $t)
                        .ok_or(ParseIntError::OutOfRange)?;
                    value = if is_positive {
                        value.checked_add(digit as $t)
                    } else {
                        value.checked_sub(digit as $t)
                    }.ok_or(ParseIntError::OutOfRange)?;
                    digit_seen = true;
                }
                if !digit_seen {
                    return Err(ParseIntError::NoDigits);
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
    fn test_parse_int_i32(#[case] s: &str, #[case] x: i32) {
        assert_eq!(parse_int::<i32>(s).unwrap(), x);
    }

    #[rstest]
    #[case("", ParseIntError::NoDigits)]
    #[case("+", ParseIntError::NoDigits)]
    #[case("-", ParseIntError::NoDigits)]
    #[case("_", ParseIntError::InvalidCharacter {c: '_', position: 0})]
    #[case("0x", ParseIntError::NoDigits)]
    #[case("0o", ParseIntError::NoDigits)]
    #[case("0b", ParseIntError::NoDigits)]
    #[case("0x+", ParseIntError::InvalidCharacter {c: '+', position: 2})]
    #[case("0o+", ParseIntError::InvalidCharacter {c: '+', position: 2})]
    #[case("0b+", ParseIntError::InvalidCharacter {c: '+', position: 2})]
    #[case("0x-", ParseIntError::InvalidCharacter {c: '-', position: 2})]
    #[case("0o-", ParseIntError::InvalidCharacter {c: '-', position: 2})]
    #[case("0b-", ParseIntError::InvalidCharacter {c: '-', position: 2})]
    #[case("0x_", ParseIntError::NoDigits)]
    #[case("0o_", ParseIntError::NoDigits)]
    #[case("0b_", ParseIntError::NoDigits)]
    #[case("0xg", ParseIntError::InvalidCharacter {c: 'g', position: 2})]
    #[case("0o9", ParseIntError::InvalidCharacter {c: '9', position: 2})]
    #[case("0b2", ParseIntError::InvalidCharacter {c: '2', position: 2})]
    #[case("feed", ParseIntError::InvalidCharacter {c: 'f', position: 0})]
    #[case(" 42 ", ParseIntError::InvalidCharacter {c: ' ', position: 0})]
    #[case("42.", ParseIntError::InvalidCharacter {c: '.', position: 2})]
    #[case("42.0", ParseIntError::InvalidCharacter {c: '.', position: 2})]
    #[case("<=>", ParseIntError::InvalidCharacter {c: '<', position: 0})]
    #[case("2147483648", ParseIntError::OutOfRange)]
    #[case("0x80000000", ParseIntError::OutOfRange)]
    #[case("0o20000000000", ParseIntError::OutOfRange)]
    #[case("0b10000000000000000000000000000000", ParseIntError::OutOfRange)]
    #[case("-2147483649", ParseIntError::OutOfRange)]
    #[case("123456789012345678902134567890", ParseIntError::OutOfRange)]
    #[case("-123456789012345678902134567890", ParseIntError::OutOfRange)]
    #[case("0X10", ParseIntError::InvalidCharacter {c: 'X', position: 1})]
    #[case("0O10", ParseIntError::InvalidCharacter {c: 'O', position: 1})]
    #[case("0B10", ParseIntError::InvalidCharacter {c: 'B', position: 1})]
    #[case("+0X10", ParseIntError::InvalidCharacter {c: 'X', position: 2})]
    #[case("+0O10", ParseIntError::InvalidCharacter {c: 'O', position: 2})]
    #[case("+0B10", ParseIntError::InvalidCharacter {c: 'B', position: 2})]
    #[case("-0X10", ParseIntError::InvalidCharacter {c: 'X', position: 2})]
    #[case("-0O10", ParseIntError::InvalidCharacter {c: 'O', position: 2})]
    #[case("-0B10", ParseIntError::InvalidCharacter {c: 'B', position: 2})]
    #[case("___1___", ParseIntError::InvalidCharacter {c: '_', position: 0})]
    #[case("_0x10", ParseIntError::InvalidCharacter {c: '_', position: 0})]
    #[case("_0o10", ParseIntError::InvalidCharacter {c: '_', position: 0})]
    #[case("_0b10", ParseIntError::InvalidCharacter {c: '_', position: 0})]
    #[case("12³45", ParseIntError::InvalidCharacter {c: '³', position: 2})]
    fn test_parse_int_i32_err(#[case] s: &str, #[case] err: ParseIntError) {
        assert_eq!(parse_int::<i32>(s).unwrap_err(), err);
    }
}
