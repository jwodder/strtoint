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
