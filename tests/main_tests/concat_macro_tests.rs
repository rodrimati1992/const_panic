use const_panic::{concat_, FmtArg};

// these tests make sure that `concat_` delegates to the same
// formatting machinery as `concat_panic`

#[test]
fn concat_basic() {
    assert_eq!(concat_!(), "");
    assert_eq!(concat_!(""), "");
    assert_eq!(concat_!("foo"), "foo");
    assert_eq!(concat_!("foo", "bar"), "foobar");
}

#[test]
fn concat_with_fmtarg() {
    assert_eq!(concat_!(FmtArg::DISPLAY; ), "");
    assert_eq!(concat_!(FmtArg::DISPLAY; [""]), "[]");
    assert_eq!(concat_!(FmtArg::DISPLAY; ["foo"]), "[foo]");
    assert_eq!(concat_!(FmtArg::DISPLAY; ["foo", "bar"]), "[foo, bar]");

    assert_eq!(concat_!(FmtArg::DEBUG; ), "");
    assert_eq!(concat_!(FmtArg::DEBUG; [""]), r#"[""]"#);
    assert_eq!(concat_!(FmtArg::DEBUG; ["foo"]), r#"["foo"]"#);
    assert_eq!(concat_!(FmtArg::DEBUG; ["foo", "bar"]), r#"["foo", "bar"]"#);
}

#[test]
fn concat_with_formatters() {
    assert_eq!(concat_!(display: [""]), "[]");
    assert_eq!(concat_!({}: ["foo"]), "[foo]");
    assert_eq!(concat_!(display: ["foo", "bar"]), "[foo, bar]");

    assert_eq!(concat_!(debug: [""]), r#"[""]"#);
    assert_eq!(concat_!({?}: ["foo"]), r#"["foo"]"#);
    assert_eq!(concat_!(debug: ["foo", "bar"]), r#"["foo", "bar"]"#);

    assert_eq!(concat_!(hex: [16u32]), "[10]");
    assert_eq!(concat_!({X}: [17u32]), "[11]");
    assert_eq!(concat_!(hex: [32u32, 33]), "[20, 21]");

    assert_eq!(concat_!(alt_hex: [16u32]), "[\n    0x10,\n]");
    assert_eq!(concat_!({#X}: [17u32]), "[\n    0x11,\n]");
    assert_eq!(concat_!(alt_hex: [32u32, 33]), "[\n    0x20,\n    0x21,\n]");

    assert_eq!(concat_!(bin: [16u32]), "[10000]");
    assert_eq!(concat_!({b}: [17u32]), "[10001]");
    assert_eq!(concat_!(bin: [32u32, 33]), "[100000, 100001]");

    assert_eq!(concat_!(alt_bin: [16u32]), "[\n    0b10000,\n]");
    assert_eq!(concat_!({#b}: [17u32]), "[\n    0b10001,\n]");
    assert_eq!(
        concat_!(alt_bin: [32u32, 33]),
        "[\n    0b100000,\n    0b100001,\n]"
    );
}
