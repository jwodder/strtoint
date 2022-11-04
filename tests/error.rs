use strtoint::StrToIntError;

#[test]
fn test_display_error_no_digits() {
    assert_eq!(StrToIntError::NoDigits.to_string(), "no digits in input");
}

#[test]
fn test_display_error_invalid_character() {
    assert_eq!(
        StrToIntError::InvalidCharacter {
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
        StrToIntError::OutOfRange.to_string(),
        "value is out of range for numeric type"
    );
}
