use const_panic::{fmt::ShortString, ArrayString, PanicVal};

#[test]
fn concat_test() {
    for strings in [
        &[""][..],
        &["hello", ""],
        &["hello", "world"][..],
        &["", "world"][..],
    ] {
        let string = ArrayString::<1024>::concat(strings);
        assert_eq!(string, *strings.concat())
    }

    assert_eq!(ArrayString::<10>::concat(&["abcd", "efghij"]), "abcdefghij");
    std::panic::catch_unwind(|| ArrayString::<10>::concat(&["abcd", "efghij", "k"])).unwrap_err();
}

#[test]
fn fmt_arraystring_test() {
    let string = "hello\nworld\r\0";
    let string_debug = r#""hello\nworld\r\x00""#;

    assert_eq!(trunc_fmt!(200; ShortString::new(string)), string_debug);
    assert_eq!(
        trunc_fmt!(200; debug: ShortString::new(string)),
        string_debug
    );
    assert_eq!(trunc_fmt!(200; display: ShortString::new(string)), string);

    let short = PanicVal::write_short_str(ShortString::new(string));
    assert_eq!(trunc_fmt!(200; short), string);
    assert_eq!(trunc_fmt!(200; debug: short), string);
    assert_eq!(trunc_fmt!(200; display: short), string);
}
