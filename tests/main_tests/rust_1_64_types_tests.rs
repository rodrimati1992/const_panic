use const_panic::{FmtArg, StdWrapper};

#[test]
#[allow(invalid_from_utf8)]
fn test_utf8_error() {
    let has_no_error_len = std::str::from_utf8(&[0xC2]).unwrap_err();
    assert_eq!(has_no_error_len.error_len(), None);

    let has_error_len = std::str::from_utf8(&[0x80]).unwrap_err();
    has_error_len.error_len().unwrap();

    test_val!(has_no_error_len, no_alternate);
    test_val!(has_error_len, no_alternate);
}
