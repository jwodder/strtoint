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
                write!(f, "invalid character {:?} at byte position {}", c, position)
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
}
