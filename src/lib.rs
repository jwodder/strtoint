use core::num::ParseIntError;
use num_traits::Num;

pub trait ParseInt {
    type Err;

    fn parse_int(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

pub fn parse_int<T: ParseInt>(s: &str) -> Result<T, <T as ParseInt>::Err> {
    T::parse_int(s)
}

macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
      $(
        impl ParseInt for $t {
            type Err = ParseIntError;

            fn parse_int(s: &str) -> Result<Self, Self::Err>
            where
                Self: Sized,
            {
                let (x, is_pos) = scan_int_str(s)?;
                if is_pos {
                    Ok(x)
                } else {
                    // Two's-complement ensures us that MAX < abs(MIN), so this
                    // shouldn't fail:
                    Ok(-x)
                }
            }
        }
      )*
    }
}

macro_rules! impl_unsigned {
    ($($t:ty),* $(,)?) => {
      $(
        impl ParseInt for $t {
            type Err = ParseIntError;

            fn parse_int(s: &str) -> Result<Self, Self::Err>
            where
                Self: Sized,
            {
                let (x, is_pos) = scan_int_str(s)?;
                if !is_pos {
                    // Parsing a negative number as an unsigned number; force an
                    // appropriate error:
                    return <Self as Num>::from_str_radix("-1", 10);
                }
                Ok(x)
            }
        }
      )*
    }
}

impl_signed!(i8, i16, i32, i64, i128, isize);
impl_unsigned!(u8, u16, u32, u64, u128, usize);

fn scan_int_str<N: Num>(s: &str) -> Result<(N, bool), <N as Num>::FromStrRadixErr> {
    // The returned bool is true for positive, false for negative
    let (s, is_pos) = {
        if let Some(t) = s.strip_prefix('+') {
            (t, true)
        } else if let Some(t) = s.strip_prefix('-') {
            (t, false)
        } else {
            (s, true)
        }
    };
    let (s, radix) = {
        if let Some(t) = s.strip_prefix("0x") {
            (t, 16)
        } else if let Some(t) = s.strip_prefix("0X") {
            (t, 16)
        } else if let Some(t) = s.strip_prefix("0o") {
            (t, 8)
        } else if let Some(t) = s.strip_prefix("0O") {
            (t, 8)
        } else if let Some(t) = s.strip_prefix("0b") {
            (t, 2)
        } else if let Some(t) = s.strip_prefix("0B") {
            (t, 2)
        } else {
            (s, 10)
        }
    };
    if s.starts_with(['+', '-']) {
        // Sign after radix prefix; force an appropriate error by parsing a
        // string with a sign in the middle:
        match <N as Num>::from_str_radix("0+", 10) {
            Err(e) => return Err(e),
            _ => unreachable!(),
        }
    }
    let x = <N as Num>::from_str_radix(s, radix)?;
    Ok((x, is_pos))
}

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
    #[case("0X10", 16)]
    #[case("0o10", 8)]
    #[case("0O10", 8)]
    #[case("0b10", 2)]
    #[case("0B10", 2)]
    #[case("+10", 10)]
    #[case("+0x10", 16)]
    #[case("+0X10", 16)]
    #[case("+0o10", 8)]
    #[case("+0O10", 8)]
    #[case("+0b10", 2)]
    #[case("+0B10", 2)]
    #[case("-10", -10)]
    #[case("-0x10", -16)]
    #[case("-0X10", -16)]
    #[case("-0o10", -8)]
    #[case("-0O10", -8)]
    #[case("-0b10", -2)]
    #[case("-0B10", -2)]
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
    fn test_parse_int_i32_err(#[case] s: &str) {
        assert!(parse_int::<i32>(s).is_err());
    }
}
