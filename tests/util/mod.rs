use strtoint::{StrToIntError, IntAccumulatorError};

pub(crate) fn assert_out_of_range(e: StrToIntError<IntAccumulatorError>) {
    assert!(matches!(e, StrToIntError<IntAccumulatorError>::Accumulator {
        source: IntAccumulatorError::OutOfRange,
        ..
    }), "expected OutOfRange error, got {:?}", e);
}
