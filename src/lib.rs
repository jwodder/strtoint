use num_traits::sign::Signed;
use num_traits::int::PrimInt;
use num_traits::Num;

pub trait ParseInt {
    type Err;

    fn parse_int(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

impl<N: PrimInt + Signed> ParseInt for N {
    type Err = <Self as Num>::FromStrRadixErr;

    fn parse_int(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        let (s, is_pos) = extract_sign(s);
        let (s, radix) = extract_radix(s);
        if s.starts_with(['+', '-']) {
            // Sign after radix prefix; force an appropriate error by parsing
            // a string with a sign in the middle:
            return <Self as Num>::from_str_radix("0+", 10);
        }
        let x = <Self as Num>::from_str_radix(s, radix)?;
        if is_pos {
            Ok(x)
        } else {
            // Two's-complement ensures us that MAX < abs(MIN), so this
            // shouldn't fail:
            Ok(-x)
        }
    }
}

impl<N: PrimInt + !Signed> ParseInt for N {
    type Err = <Self as Num>::FromStrRadixErr;

    fn parse_int(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        let (s, is_pos) = extract_sign(s);
        if !is_pos {
            // Parsing a negative number as an unsigned number; force an
            // appropriate error:
            return <Self as Num>::from_str_radix("-1", 10);
        }
        let (s, radix) = extract_radix(s);
        if s.starts_with(['+', '-']) {
            // Sign after radix prefix; force an appropriate error by parsing
            // a string with a sign in the middle:
            return <Self as Num>::from_str_radix("0+", 10);
        }
        <Self as Num>::from_str_radix(s, radix)
    }
}

fn extract_sign(s: &str) -> (&str, bool) {
    // bool is true for positive, false for negative
    if let Some(t) = s.strip_prefix("+") {
        (t, true)
    } else if let Some(t) = s.strip_prefix("-") {
        (t, false)
    } else {
        (s, true)
    }
}

fn extract_radix(s: &str) -> (&str, u32) {
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
}
