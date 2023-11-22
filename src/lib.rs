#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! `strtoint` provides a function of the same name for parsing integer
//! literals from strings, with support for the base prefixes `0x`, `0o`, and
//! `0b` for hexadecimal, octal, and binary literals, respectively.
//!
//! This crate supports parsing into all primitive integer types built into
//! Rust, along with their "NonZero" equivalents.
//!
//! If the `std` feature (enabled by default) is disabled, this crate will be
//! built in no-std mode.  The only difference is that [`StrToIntError`] only
//! implements the [`std::error::Error`] trait under `std`.
//!
//! Examples
//! ========
//!
//! ```
//! use core::num::NonZeroUsize;
//! use strtoint::strtoint;
//!
//! assert_eq!(strtoint::<i32>("123").unwrap(), 123);
//! assert_eq!(strtoint::<u32>("0xabcd_FFFF").unwrap(), 2882404351);
//! assert_eq!(strtoint::<i16>("0o644").unwrap(), 420);
//! assert_eq!(strtoint::<i8>("-0b00101010").unwrap(), -42);
//! assert!(strtoint::<i64>("42.0").is_err());
//!
//! assert_eq!(
//!     strtoint::<NonZeroUsize>("123_456").unwrap(),
//!     NonZeroUsize::new(123456).unwrap()
//! );
//! assert!(strtoint::<NonZeroUsize>("0").is_err());
//! ```
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
/// This function is implemented for all primitive integer types built into
/// Rust, along with their "NonZero" equivalents, and the `Err` type for all of
/// them is [`StrToIntError`].
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
pub fn strtoint<T: StrToInt>(
    s: &str,
) -> Result<T, StrToIntError<<<T as StrToInt>::Accumulator as Accumulator<T>>::Err>> {
    T::strtoint(s)
}

/// Trait used to implement the [`strtoint()`] function
///
/// Call [`strtoint()`] instead of using this trait directly.  You only ever
/// need to import this trait if you're implementing support for a custom
/// numeric type in your own crate.
pub trait StrToInt: Sized {
    type Accumulator: Accumulator<Self>;

    fn positive_from_digits<I: IntoIterator<Item = u32>>(
        base: Base,
        digits: I,
    ) -> Result<Self, <Self::Accumulator as Accumulator<Self>>::Err> {
        let mut acc = Self::Accumulator::start_positive(base)?;
        for d in digits {
            acc.push_digit(d)?;
        }
        acc.finish()
    }

    fn negative_from_digits<I: IntoIterator<Item = u32>>(
        base: Base,
        digits: I,
    ) -> Result<Self, <Self::Accumulator as Accumulator<Self>>::Err> {
        let mut acc = Self::Accumulator::start_negative(base)?;
        for d in digits {
            acc.push_digit(d)?;
        }
        acc.finish()
    }

    /// Parse a string as the type in question
    fn strtoint(
        mut s: &str,
    ) -> Result<Self, StrToIntError<<Self::Accumulator as Accumulator<Self>>::Err>> {
        let mut offset = 0;
        let is_positive = {
            if let Some(t) = s.strip_prefix('+') {
                offset += 1;
                s = t;
                true
            } else if let Some(t) = s.strip_prefix('-') {
                offset += 1;
                s = t;
                false
            } else {
                true
            }
        };
        let base = {
            if let Some(t) = s.strip_prefix("0x") {
                offset += 2;
                s = t;
                Base::HEXADECIMAL
            } else if let Some(t) = s.strip_prefix("0o") {
                offset += 2;
                s = t;
                Base::OCTAL
            } else if let Some(t) = s.strip_prefix("0b") {
                offset += 2;
                s = t;
                Base::BINARY
            } else {
                Base::DECIMAL
            }
        };
        let mut acc = if is_positive {
            Self::Accumulator::start_positive(base)
        } else {
            Self::Accumulator::start_negative(base)
        }
        .map_err(|source| StrToIntError::Accumulator {
            source,
            position: offset,
        })?;
        let mut digit_seen = false;
        let mut position = offset;
        for (i, c) in s.char_indices() {
            position = offset + i;
            if c == '_' {
                if !digit_seen && base == Base::DECIMAL {
                    return Err(StrToIntError::InvalidCharacter { c, position });
                }
                continue;
            }
            let digit = base
                .parse_digit(c)
                .ok_or(StrToIntError::InvalidCharacter { c, position })?;
            acc.push_digit(digit)
                .map_err(|source| StrToIntError::Accumulator { source, position })?;
            digit_seen = true;
        }
        if !digit_seen {
            return Err(StrToIntError::NoDigits);
        }
        acc.finish()
            .map_err(|source| StrToIntError::Accumulator { source, position })
    }
}

/// Error type for the [`strtoint()`] function
///
/// This type is used as the error type for [`strtoint()`] and [`StrToInt`] for
/// all types covered by this crate.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum StrToIntError<E> {
    /// Returned when the input string contained no digits
    NoDigits,
    /// Returned when the input string contained an invalid character; `c` is
    /// the character in question, and `position` is its index in the input
    InvalidCharacter {
        c: char,
        position: usize,
    },
    Accumulator {
        source: E,
        position: usize,
    },
}

impl<E: fmt::Display> fmt::Display for StrToIntError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrToIntError::NoDigits => write!(f, "no digits in input"),
            StrToIntError::InvalidCharacter { c, position } => {
                write!(f, "invalid character {:?} at position {}", c, position)
            }
            StrToIntError::Accumulator { source, position } => {
                write!(f, "numeric error at position {}: {}", position, source)
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<E: std::error::Error + 'static> std::error::Error for StrToIntError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StrToIntError::Accumulator { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub trait Accumulator<Int>: Sized {
    type Err;

    fn start_positive(base: Base) -> Result<Self, Self::Err>;

    // This method should error when `Int` is unsigned.
    fn start_negative(base: Base) -> Result<Self, Self::Err>;

    // This method is called when parsing an integer string in a context where
    // "`-0`" is acceptable.
    // This method should be overwritten by accumulators for types that can be
    // zero but not negative.
    fn start_nonpositive(base: Base) -> Result<Self, Self::Err> {
        Self::start_negative(base)
    }

    fn push_digit(&mut self, digit: u32) -> Result<(), Self::Err>;

    fn finish(self) -> Result<Int, Self::Err>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IntAccumulator<Int> {
    base: Base,
    cast_base: Int,
    value: Int,
    negative: bool,
    any_digits: bool,
}

macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
      $(
        impl StrToInt for $t {
            type Accumulator = IntAccumulator<$t>;
        }

        impl Accumulator<$t> for IntAccumulator<$t> {
            type Err = IntAccumulatorError;

            fn start_positive(base: Base) -> Result<Self, IntAccumulatorError> {
                Ok(IntAccumulator {
                    base,
                    cast_base: base.get() as $t,
                    value: 0,
                    negative: false,
                    any_digits: false,
                })
            }

            fn start_negative(base: Base) -> Result<Self, IntAccumulatorError> {
                Ok(IntAccumulator {
                    base,
                    cast_base: base.get() as $t,
                    value: 0,
                    negative: true,
                    any_digits: false,
                })
            }

            fn push_digit(&mut self, digit: u32) -> Result<(), IntAccumulatorError> {
                if digit >= self.base.get() {
                    return Err(IntAccumulatorError::DigitTooLarge {
                        base: self.base,
                        digit,
                    });
                }
                let digit = digit as $t;
                self.value = self
                    .value
                    .checked_mul(self.cast_base)
                    .and_then(|v| {
                        if self.negative {
                            v.checked_sub(digit)
                        } else {
                            v.checked_add(digit)
                        }
                    })
                    .ok_or(IntAccumulatorError::OutOfRange)?;
                self.any_digits = true;
                Ok(())
            }

            fn finish(self) -> Result<$t, IntAccumulatorError> {
                if self.any_digits {
                    Ok(self.value)
                } else {
                    Err(IntAccumulatorError::NoDigits)
                }
            }
        }
      )*
    }
}

macro_rules! impl_unsigned {
    ($($t:ty),* $(,)?) => {
      $(
        impl StrToInt for $t {
            type Accumulator = IntAccumulator<$t>;
        }

        impl Accumulator<$t> for IntAccumulator<$t> {
            type Err = IntAccumulatorError;

            fn start_positive(base: Base) -> Result<Self, IntAccumulatorError> {
                Ok(IntAccumulator {
                    base,
                    cast_base: base.get() as $t,
                    value: 0,
                    negative: false,
                    any_digits: false,
                })
            }

            fn start_negative(_base: Base) -> Result<Self, IntAccumulatorError> {
                Err(IntAccumulatorError::CannotBeNegative)
            }

            fn start_nonpositive(base: Base) -> Result<Self, Self::Err> {
                Ok(IntAccumulator {
                    base,
                    cast_base: base.get() as $t,
                    value: 0,
                    negative: true,
                    any_digits: false,
                })
            }

            fn push_digit(&mut self, digit: u32) -> Result<(), IntAccumulatorError> {
                if digit >= self.base.get() {
                    return Err(IntAccumulatorError::DigitTooLarge {
                        base: self.base,
                        digit,
                    });
                }
                let digit = digit as $t;
                self.value = self
                    .value
                    .checked_mul(self.cast_base)
                    .and_then(|v| {
                        if self.negative {
                            v.checked_sub(digit)
                        } else {
                            v.checked_add(digit)
                        }
                    })
                    .ok_or(IntAccumulatorError::OutOfRange)?;
                self.any_digits = true;
                Ok(())
            }

            fn finish(self) -> Result<$t, IntAccumulatorError> {
                if self.any_digits {
                    Ok(self.value)
                } else {
                    Err(IntAccumulatorError::NoDigits)
                }
            }
        }
      )*
    }
}

impl_signed!(i8, i16, i32, i64, i128, isize);
impl_unsigned!(u8, u16, u32, u64, u128, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NonZeroAccumulator<A>(A);

macro_rules! impl_nonzero {
    ($t:ty, $inner:ty) => {
        impl StrToInt for $t {
            type Accumulator = NonZeroAccumulator<IntAccumulator<$inner>>;
        }

        impl Accumulator<$t> for NonZeroAccumulator<IntAccumulator<$inner>> {
            type Err = IntAccumulatorError;

            fn start_positive(base: Base) -> Result<Self, IntAccumulatorError> {
                IntAccumulator::<$inner>::start_positive(base).map(NonZeroAccumulator)
            }

            fn start_negative(base: Base) -> Result<Self, IntAccumulatorError> {
                IntAccumulator::<$inner>::start_negative(base).map(NonZeroAccumulator)
            }

            // Do not override `start_nonpositive()`.  Unsigned nonzero types
            // should error when it's called.

            fn push_digit(&mut self, digit: u32) -> Result<(), IntAccumulatorError> {
                self.0.push_digit(digit)
            }

            fn finish(self) -> Result<$t, IntAccumulatorError> {
                self.0
                    .finish()
                    .and_then(|n| <$t>::new(n).ok_or(IntAccumulatorError::OutOfRange))
            }
        }
    };
}

impl_nonzero!(core::num::NonZeroI8, i8);
impl_nonzero!(core::num::NonZeroI16, i16);
impl_nonzero!(core::num::NonZeroI32, i32);
impl_nonzero!(core::num::NonZeroI64, i64);
impl_nonzero!(core::num::NonZeroI128, i128);
impl_nonzero!(core::num::NonZeroIsize, isize);
impl_nonzero!(core::num::NonZeroU8, u8);
impl_nonzero!(core::num::NonZeroU16, u16);
impl_nonzero!(core::num::NonZeroU32, u32);
impl_nonzero!(core::num::NonZeroU64, u64);
impl_nonzero!(core::num::NonZeroU128, u128);
impl_nonzero!(core::num::NonZeroUsize, usize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum IntAccumulatorError {
    NoDigits,
    OutOfRange,
    DigitTooLarge { base: Base, digit: u32 },
    CannotBeNegative,
}

impl fmt::Display for IntAccumulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntAccumulatorError::NoDigits => write!(f, "no digits provided to accumulator"),
            IntAccumulatorError::OutOfRange => {
                write!(f, "accumulated value is out of range for numeric type")
            }
            IntAccumulatorError::DigitTooLarge { base, digit } => write!(
                f,
                "accumulator received digit {} out of range for base {}",
                digit,
                base.get()
            ),
            IntAccumulatorError::CannotBeNegative => {
                write!(f, "cannot start negative value for unsigned numeric type")
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for IntAccumulatorError {}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Base(u32);

impl Base {
    pub const DECIMAL: Base = Base(10);
    pub const HEXADECIMAL: Base = Base(16);
    pub const OCTAL: Base = Base(8);
    pub const BINARY: Base = Base(2);

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn parse_digit(&self, c: char) -> Option<u32> {
        c.to_digit(self.0)
    }
}

impl TryFrom<u32> for Base {
    type Error = BaseError;

    fn try_from(value: u32) -> Result<Base, BaseError> {
        match value {
            2..=36 => Ok(Base(value)),
            _ => Err(BaseError),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BaseError;

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "integer base must be between 2 and 36, inclusive")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for BaseError {}

pub type StrToPrimIntError = StrToIntError<IntAccumulatorError>;
