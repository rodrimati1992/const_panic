use const_panic::FmtArg;

const DISPLAY: FmtArg = FmtArg::DISPLAY;
const DISPLAY_TUP: (FmtArg,) = (FmtArg::DISPLAY,);
const DEBUG_TUP2: (((), FmtArg),) = (((), FmtArg::DEBUG),);

#[test]
fn concat_panic_args_test() {
    let string = "\nfoo\"\r";
    let string_debug = r#""\nfoo\"\r""#;

    assert_eq!(
        trunc_fmt!(999; ::const_panic::FmtArg::DEBUG; "\nfoo\"\r"),
        string
    );
    assert_eq!(trunc_fmt!(999; FmtArg::DEBUG; string), string_debug);

    assert_eq!(trunc_fmt!(999; DISPLAY; "\nfoo\"\r"), string);
    assert_eq!(trunc_fmt!(999; DISPLAY_TUP.0; string), string);
    assert_eq!(trunc_fmt!(999; DEBUG_TUP2.0.1; string), string_debug);

    assert_eq!(trunc_fmt!(999; FmtArg::DISPLAY; "\nfoo\"\r"), string);
    assert_eq!(
        trunc_fmt!(999; const_panic::FmtArg::DISPLAY; string),
        string
    );

    assert_eq!(trunc_fmt!(999; "\nfoo\"\r"), string);
    assert_eq!(trunc_fmt!(999; string), string_debug);

    assert_eq!(trunc_fmt!(999; debug: "\nfoo\"\r"), string_debug);
    assert_eq!(trunc_fmt!(999; debug: string), string_debug);

    assert_eq!(trunc_fmt!(999; display: "\nfoo\"\r"), string);
    assert_eq!(trunc_fmt!(999; display: string), string);
}

macro_rules! fmt_flatten {
    ($($args:tt)*) => (
        const_panic::ArrayString::<256>::from_panicvals(
            &const_panic::flatten_panicvals!($($args)*)
        ).unwrap()
    )
}

#[test]
fn flatten_panicvals_args_test() {
    let string = "\nfoo\"\r";
    let string_debug = r#""\nfoo\"\r""#;

    assert_eq!(
        fmt_flatten!(::const_panic::FmtArg::DEBUG; "\nfoo\"\r"),
        string
    );
    assert_eq!(
        fmt_flatten!(const_panic::FmtArg::DEBUG; string),
        string_debug
    );

    assert_eq!(fmt_flatten!(DISPLAY; "\nfoo\"\r"), string);
    assert_eq!(fmt_flatten!(DISPLAY_TUP.0; string), string);
    assert_eq!(fmt_flatten!(DEBUG_TUP2.0.1; string), string_debug);

    assert_eq!(fmt_flatten!(FmtArg::DISPLAY; "\nfoo\"\r"), string);
    assert_eq!(fmt_flatten!(FmtArg::DISPLAY; string), string);

    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; debug: "\nfoo\"\r"),
        string_debug
    );
    assert_eq!(fmt_flatten!(FmtArg::DEBUG; debug: string), string_debug);

    assert_eq!(fmt_flatten!(FmtArg::DEBUG; display: "\nfoo\"\r"), string);
    assert_eq!(fmt_flatten!(FmtArg::DEBUG; display: string), string);

    assert_eq!(fmt_flatten!(FmtArg::DEBUG; &str => "\nfoo\"\r"), string);
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; &str => display: "\nfoo\"\r"),
        string
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; &str => debug: "\nfoo\"\r"),
        string_debug
    );
    assert_eq!(fmt_flatten!(FmtArg::DEBUG; &str => string), string_debug);
    assert_eq!(fmt_flatten!(FmtArg::DISPLAY; &str => string), string);
    assert_eq!(fmt_flatten!(FmtArg::DEBUG; &str => display: string), string);
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; &str => debug: string),
        string_debug
    );
}
