[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip) <!-- [![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active) -->
[![CI Status](https://github.com/jwodder/strtoint/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/strtoint/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/strtoint/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/strtoint)
[![MIT License](https://img.shields.io/github/license/jwodder/strtoint.svg)](https://opensource.org/licenses/MIT)

[GitHub](https://github.com/jwodder/strtoint) | [crates.io](https://crates.io/crates/strtoint) | [Documentation](https://docs.rs/strtoint) | [Issues](https://github.com/jwodder/strtoint/issues)

`strtoint` provides a function of the same name for parsing integer literals
from strings, with support for the base prefixes `0x`, `0o`, and `0b` for
hexadecimal, octal, and binary literals, respectively.

This crate supports parsing into all primitive integer types built into Rust,
along with their "NonZero" equivalents.

If the `std` feature (enabled by default) is disabled, this crate will be built
in no-std mode.  The only difference is that `StrToIntError` only implements
the `std::error::Error` trait under `std`.

Installation
============

`strtoint` requires version 1.56 of Rust or higher.  To use the `strtoint`
library in your Cargo project, add the following to your `Cargo.toml`:

```toml
[dependencies]
strtoint = "0.1.0-alpha"
```


Examples
========

```rust
use core::num::NonZeroUsize;
use strtoint::strtoint;

assert_eq!(strtoint::<i32>("123").unwrap(), 123);
assert_eq!(strtoint::<u32>("0xabcd_FFFF").unwrap(), 2882404351);
assert_eq!(strtoint::<i16>("0o644").unwrap(), 420);
assert_eq!(strtoint::<i8>("-0b00101010").unwrap(), -42);
assert!(strtoint::<i64>("42.0").is_err());

assert_eq!(
    strtoint::<NonZeroUsize>("123_456").unwrap(),
    NonZeroUsize::new(123456).unwrap()
);
assert!(strtoint::<NonZeroUsize>("0").is_err());
```
