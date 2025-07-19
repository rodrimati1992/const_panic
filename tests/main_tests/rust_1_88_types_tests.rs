use core::ffi::c_str::{CStr, FromBytesUntilNulError, FromBytesWithNulError};

#[test]
#[cfg(feature = "non_basic")]
fn test_from_bytes_with_nul_error() {
    for err in [
        FromBytesWithNulError::InteriorNul { position: 0 },
        FromBytesWithNulError::InteriorNul { position: 10 },
        FromBytesWithNulError::NotNulTerminated,
    ] {
        // no_display: Rust 1.90 changes the Display message
        test_val! {err, no_display}
    }
}

#[test]
#[cfg(feature = "non_basic")]
fn test_from_bytes_until_nul_error() {
    let err: FromBytesUntilNulError = CStr::from_bytes_until_nul(&[]).unwrap_err();

    test_val! {err}
}
