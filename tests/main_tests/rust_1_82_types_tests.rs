use core::ffi::c_str::{CStr, FromBytesUntilNulError, FromBytesWithNulError};
use core::num::{IntErrorKind, ParseIntError};

use const_panic::{FmtArg, StdWrapper};

macro_rules! test_val {
    ($value:expr) => ({
        let val = $value;

        {
            let debug = format!("{:?}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DEBUG)),
                &*debug,
            );
        }

        {
            let alt_debug = format!("{:#?}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::ALT_DEBUG)),
                &*alt_debug,
            );
        }

        {
            let display = format!("{}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DISPLAY)),
                &*display,
            );
        }

        {
            let alt_display = format!("{:#}", val);
            assert_eq!(
                trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::ALT_DISPLAY)),
                &*alt_display,
            );
        }
    })
}

#[test]
fn test_parse_int_error() {
    {
        let err: ParseIntError = u32::from_str_radix("", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::Empty);
        test_val! {err}
    }
    {
        let err: ParseIntError = u32::from_str_radix("A", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::InvalidDigit);
        test_val! {err}
    }
    {
        let err: ParseIntError = u8::from_str_radix("256", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::PosOverflow);
        test_val! {err}
    }
    {
        let err: ParseIntError = i8::from_str_radix("-256", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::NegOverflow);
        test_val! {err}
    }
    {
        let err: ParseIntError = "0".parse::<core::num::NonZeroI8>().unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::Zero);
        test_val! {err}
    }
}

#[test]
fn test_int_error_kind() {
    for kind in [
        IntErrorKind::Empty,
        IntErrorKind::InvalidDigit,
        IntErrorKind::PosOverflow,
        IntErrorKind::NegOverflow,
        IntErrorKind::Zero,
    ] {
        assert_eq!(
            trunc_fmt!(1024; StdWrapper(&kind).to_panicvals(FmtArg::DEBUG)),
            &*format!("{kind:?}"),
        );
    }
}

#[test]
#[cfg(feature = "non_basic")]
fn test_from_bytes_with_nul_error() {
    for err in [
        FromBytesWithNulError::InteriorNul { position: 0 },
        FromBytesWithNulError::InteriorNul { position: 10 },
        FromBytesWithNulError::NotNulTerminated,
    ] {
        test_val! {err}
    }
}

#[test]
#[cfg(feature = "non_basic")]
fn test_from_bytes_until_nul_error() {
    let err: FromBytesUntilNulError = CStr::from_bytes_until_nul(&[]).unwrap_err();

    test_val! {err}
}
