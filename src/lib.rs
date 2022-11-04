//! Parse integers from strings, with support for base prefixes
//!
//! `strtoint` provides a function of the same name for parsing integer
//! literals from strings, with support for the base prefixes `0x`, `0o`, and
//! `0b` for hexadecimal, octal, and binary literals, respectively.
//!
//! If the `std` feature (enabled by default) is disabled, this crate will be
//! built in no-std mode.  The only difference is that [`StrToIntError`] only
//! implements the [`std::error::Error`] trait under `std`.
//!
//! ```
//! use strtoint::strtoint;
//!
//! assert_eq!(strtoint::<i32>("123").unwrap(), 123);
//! assert_eq!(strtoint::<u32>("0xabcd_FFFF").unwrap(), 2882404351);
//! assert_eq!(strtoint::<i16>("0o644").unwrap(), 420);
//! assert_eq!(strtoint::<i8>("-0b00101010").unwrap(), -42);
//! assert!(strtoint::<i64>("42.0").is_err());
//! ```
#![no_std]
use core::fmt;

#[cfg(feature = "std")]
extern crate std;

/// Parse an integer from a string.
///
/// This function follows the same rules as for [Rust's integer literals][1],
/// with support for signs and without support for integer suffixes.
/// Specifically, a valid integer string is an optional sign (`+` or `-`, the
/// latter forbidden for unsigned types), followed by an optional base prefix
/// (`0x`, `0o`, or `0b`, all lowercase), followed by one or more digits
/// optionally interspersed with underscores.  Leading & trailing whitespace is
/// not allowed.
///
/// [1]: https://doc.rust-lang.org/stable/reference/tokens.html#integer-literals
///
/// This function is implemented for all primitive integer types built in to
/// Rust, and the `Err` type for all of them is [`StrToIntError`].
///
/// # Errors
///
/// This function will return an error under the following conditions:
///
/// - The input string does not contain any digits after the optional sign and
///   base prefix
/// - The input string contains an invalid character, including surrounding or
///   internal whitespace, an invalid digit for the base in question, an
///   invalid base prefix, a sign after a base prefix, or a `-` sign for an
///   unsigned type
/// - The numeric value represented by the string is outside the range of valid
///   values for the numeric type
pub fn strtoint<T: StrToInt>(s: &str) -> Result<T, <T as StrToInt>::Err> {
    T::strtoint(s)
}

/// Trait used to implement the [`strtoint()`] function
///
/// Call [`strtoint()`] instead of using this trait directly.  You only ever
/// need to import this trait if you're implementing support for a custom
/// numeric type in your own crate.
pub trait StrToInt {
    type Err;

    /// Parse a string as the type in question
    fn strtoint(s: &str) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

/// Error type for the [`strtoint()`] function
///
/// This type is used as the error type for [`strtoint()`] and [`StrToInt`] for
/// all types covered by this crate.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum StrToIntError {
    /// Returned when the input string contained no digits
    NoDigits,
    /// Returned when the input string contained an invalid character; `c` is
    /// the character in question, and `position` is its index in the input
    InvalidCharacter { c: char, position: usize },
    /// Returned when the numeric value of the input string was out of range
    /// for the numeric type
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

#[cfg(feature = "std")]
impl std::error::Error for StrToIntError {}

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
