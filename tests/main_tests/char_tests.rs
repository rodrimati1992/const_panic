//! There's additional tests in const_panic::fmt::char_formatting::tests.

use const_panic::{FmtArg, StdWrapper};

macro_rules! test_val {
    ($value:expr) => ({
        let val = $value;
        let display = format!("{}", val);
        let debug = if val > '\u{7E}' {
            format!("'{}'", val)
        } else {
            format!("{:?}", val)
        };
        assert_eq!(
            trunc_fmt!(32; StdWrapper(&val).to_panicvals(FmtArg::DEBUG)),
            &*debug,
            "debug",
        );
        assert_eq!(
            trunc_fmt!(32; StdWrapper(&val).to_panicvals(FmtArg::DISPLAY)),
            &*display,
            "display",
        );
    })
}

#[test]
fn basic_char_tests() {
    for c in (' '..='\u{FFF}').chain([char::MAX]) {
        test_val! {c}
    }
}
