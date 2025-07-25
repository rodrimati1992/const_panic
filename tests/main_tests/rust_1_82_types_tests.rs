use core::num::{IntErrorKind, ParseIntError};

use const_panic::{FmtArg, StdWrapper};

#[test]
fn test_parse_int_error() {
    {
        let err: ParseIntError = u32::from_str_radix("", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::Empty);
        test_val! {err, no_alternate}
    }
    {
        let err: ParseIntError = u32::from_str_radix("A", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::InvalidDigit);
        test_val! {err, no_alternate}
    }
    {
        let err: ParseIntError = u8::from_str_radix("256", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::PosOverflow);
        test_val! {err, no_alternate}
    }
    {
        let err: ParseIntError = i8::from_str_radix("-256", 10).unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::NegOverflow);
        test_val! {err, no_alternate}
    }
    {
        let err: ParseIntError = "0".parse::<core::num::NonZeroI8>().unwrap_err();
        assert_eq!(*err.kind(), IntErrorKind::Zero);
        test_val! {err, no_alternate}
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
