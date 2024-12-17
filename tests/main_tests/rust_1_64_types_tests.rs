use const_panic::{FmtArg, StdWrapper};

macro_rules! test_val {
    ($value:expr) => ({
        let val = $value;
        let display = format!("{}", val);
        let debug = format!("{:?}", val);
        assert_eq!(
            trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DEBUG)),
            &*debug,
        );
        assert_eq!(
            trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DISPLAY)),
            &*display,
        );
    })
}

#[test]
#[allow(invalid_from_utf8)]
fn test_utf8_error() {
    let has_no_error_len = std::str::from_utf8(&[0xC2]).unwrap_err();
    assert_eq!(has_no_error_len.error_len(), None);

    let has_error_len = std::str::from_utf8(&[0x80]).unwrap_err();
    has_error_len.error_len().unwrap();

    test_val!(has_no_error_len);
    test_val!(has_error_len);
}
