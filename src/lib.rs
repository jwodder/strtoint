use core::iter::Iterator;

pub trait ParseInt {
    type Err;

    fn parse_int(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

pub fn parse_int<T: ParseInt>(s: &str) -> Result<T, <T as ParseInt>::Err> {
    T::parse_int(s)
}

#[derive(Debug)]
pub enum ParseIntError {
    NoDigits,
    InvalidCharacter { c: char, position: usize },
    OutOfRange,
}

// TODO: Impl Display and Error for ParseIntError

macro_rules! implement {
    ($($t:ty),* $(,)?) => {
      $(
        impl ParseInt for $t {
            type Err = ParseIntError;

            fn parse_int(mut s: &str) -> Result<Self, Self::Err>
            where
                Self: Sized,
            {
                let mut pos = 0;
                let is_positive = {
                    if let Some(t) = s.strip_prefix('+') {
                        pos += 1;
                        s = t;
                        true
                    } else if let Some(t) = s.strip_prefix('-') {
                        if <$t>::MIN == 0 {
                            return Err(ParseIntError::InvalidCharacter {c: '-', position: pos});
                        }
                        pos += 1;
                        s = t;
                        false
                    } else {
                        true
                    }
                };
                let radix = {
                    if let Some(t) = s.strip_prefix("0x") {
                        pos += 2;
                        s = t;
                        16
                    } else if let Some(t) = s.strip_prefix("0o") {
                        pos += 2;
                        s = t;
                        8
                    } else if let Some(t) = s.strip_prefix("0b") {
                        pos += 2;
                        s = t;
                        2
                    } else {
                        10
                    }
                };

                let mut value: $t = 0;
                let mut digit_seen = false;
                for (i, &b) in (pos..).into_iter().zip(s.as_bytes()) {
                    if b == b'_' {
                        continue;
                    }
                    let digit = (b as char).to_digit(radix).ok_or_else(|| ParseIntError::InvalidCharacter {c: b as char, position: i})?;
                    value = value.checked_mul(radix as $t).ok_or(ParseIntError::OutOfRange)?;
                    if is_positive {
                        value = value.checked_add(digit as $t).ok_or(ParseIntError::OutOfRange)?
                    } else {
                        value = value.checked_sub(digit as $t).ok_or(ParseIntError::OutOfRange)?
                    }
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
    fn test_parse_int_i32(#[case] s: &str, #[case] x: i32) {
        assert_eq!(parse_int::<i32>(s).unwrap(), x);
    }

    #[rstest]
    #[case("")]
    #[case("+")]
    #[case("-")]
    #[case("0x")]
    #[case("0o")]
    #[case("0b")]
    #[case("0x+")]
    #[case("0o+")]
    #[case("0b+")]
    #[case("0x-")]
    #[case("0o-")]
    #[case("0b-")]
    #[case("0xg")]
    #[case("0o9")]
    #[case("0b2")]
    #[case("feed")]
    #[case(" 42 ")]
    #[case("42.")]
    #[case("42.0")]
    #[case("<=>")]
    #[case("123456789012345678902134567890")]
    #[case("-123456789012345678902134567890")]
    #[case("0X10")]
    #[case("0O10")]
    #[case("0B10")]
    #[case("+0X10")]
    #[case("+0O10")]
    #[case("+0B10")]
    #[case("-0X10")]
    #[case("-0O10")]
    #[case("-0B10")]
    fn test_parse_int_i32_err(#[case] s: &str) {
        assert!(parse_int::<i32>(s).is_err());
    }
}
