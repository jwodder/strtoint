use strtoint::{IntAccumulatorError, StrToPrimIntError};

#[test]
fn test_display_error_no_digits() {
    assert_eq!(
        StrToPrimIntError::NoDigits.to_string(),
        "no digits in input"
    );
}

#[test]
fn test_display_error_invalid_character() {
    assert_eq!(
        StrToPrimIntError::InvalidCharacter {
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
        StrToPrimIntError::Accumulator {
            source: IntAccumulatorError::OutOfRange,
            position: 5
        }
        .to_string(),
        "numeric error at position 2: accumulated value is out of range for numeric type"
    );
}
